use chrono::Utc;
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
// use std::sync::Arc;
use tokio::spawn;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use crate::config::{BotConfig, CompletedScheduled, QuizData, ScheduledMessage};
use crate::utils::{
    add_attachments, add_poll, contains_answer, get_target_channel, get_target_channel_id,
    get_target_guild, remove_ongoing_quiz, save_bot_config, set_ongoing_quiz, sleep_remaining_time,
};
use crate::OngoingQuiz;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        spawn(async move {
            Self::start_ticking(ctx).await;
        });

        info!("The bot is ready");
    }

    async fn message(&self, ctx: Context, new_message: Message) {
        let target_channel = get_target_channel_id(&ctx).await;
        if new_message.channel_id != target_channel {
            return;
        }

        let mut quiz_done = false;

        'block: {
            // Intentionally maintain lock on Mutex for an extended period of time
            // so incase of high message volume, two messages
            // doesn't get declared as winner while the original one is updating

            let data_read = ctx.data.read().await;
            let data = data_read.get::<OngoingQuiz>().unwrap();
            let ongoing_quiz = data.lock().await;

            if ongoing_quiz.is_none() {
                return;
            }

            let quiz_data = ongoing_quiz.clone().unwrap();
            let quiz_answer = quiz_data.answer();

            // Cancel ongoing quiz if a timing is provided and exceeded
            if let Some(end_time) = quiz_data.end_at() {
                let now = Utc::now();

                if &now >= end_time {
                    info!("Quiz end time has been reached. Cancelling ongoing quiz.");
                    quiz_done = true;
                    break 'block;
                }
            }

            // Replace various character that are commonly used and including and excluding them
            // both are acceptable
            let message_content = new_message
                .content
                .to_lowercase()
                .replace(['\n', '-', ',', '.', '!', '?', ':', '-'], " ");

            let answer_content = quiz_answer
                .to_lowercase()
                .replace(['\n', '-', ',', '.', '!', '?', ':', '-'], " ");

            // Filter out any space only words after the split. Example: this. is something
            // After . is replaced with " ", there would be two simultaneous space only word
            // that are not useful
            let split_content = message_content
                .split_whitespace()
                .filter(|s| !s.trim().is_empty())
                .collect::<Vec<_>>();
            let split_answer = answer_content
                .split_whitespace()
                .filter(|s| !s.trim().is_empty())
                .collect::<Vec<_>>();

            if contains_answer(split_content, split_answer) {
                info!("Quiz answer found in message: {}", new_message.content);
                quiz_done = true;

                let result = new_message.reply(&ctx, quiz_data.reply_with()).await;
                if let Err(e) = result {
                    error!("Failed to send the reply to the winner. This will be considered as completed regardless. Reason: {e}")
                }
            }
        }
        if quiz_done {
            remove_ongoing_quiz(&ctx).await;
        }
    }
}

impl Handler {
    async fn start_ticking(ctx: Context) {
        let bot_config = BotConfig::get_config();

        if let Err(e) = bot_config {
            error!("Error reading the bot config. Exiting. Error: {e}");
            std::process::exit(1);
        }
        let mut config = bot_config.unwrap();

        // Wait 5 seconds for the cache to load
        info!("Waiting for cache to load");
        sleep(Duration::from_secs(5)).await;

        let target_guild_name = config.get_target_guild();
        let target_channel_name = config.get_target_channel();
        let pin_all = config.pin_all();

        info!(
            "Target guild name: {}, Target channel name: {}",
            target_guild_name, target_channel_name
        );

        let mut target_guild = None;
        let mut target_channel = None;

        // Fetch both guild and channel in a single flow. In case the guild channel list gets
        // updated and the target channel is part of that, this should ensure the search isn't
        // happening on stale guild data
        while target_guild.is_none() && target_channel.is_none() {
            target_guild = get_target_guild(&ctx, &target_guild_name);

            if let Some(guild) = &target_guild {
                target_channel = get_target_channel(guild.clone(), &target_channel_name);

                if target_channel.is_none() {
                    error!("Target channel not found. Trying again in 60 seconds");
                }
            } else {
                error!("Target guild not found. Trying again in 60 seconds");
            }
        }

        // let target_guild = target_guild.unwrap();
        let target_channel = target_channel.unwrap();
        config.set_target_channel_id(target_channel.0);
        info!("Target channel found");

        save_bot_config(&ctx, config).await;
        info!("Starting scheduling.");

        loop {
            // For most thing that fails during this loop, sleep till the current minute ends and try
            // again the next minute

            let schedule_data = ScheduledMessage::get_all_scheduled_messages();
            let completed_data = CompletedScheduled::get_completed_scheduled();

            if let Err(e) = &schedule_data {
                error!("Failed to read scheduled message data. Reason: {e}");
                sleep_remaining_time().await;
                continue;
            }

            if let Err(e) = &completed_data {
                error!("Failed to read message completion data. Reason: {e}");
                sleep_remaining_time().await;
                continue;
            }

            let scheduled = schedule_data.unwrap();
            let mut completed = completed_data.unwrap();
            let mut to_handle = Vec::new();

            let now = Utc::now();

            for message in &scheduled {
                if !completed.completed.contains(&message.id()) && now >= message.scheduled_at() {
                    to_handle.push(message);
                }
            }

            if to_handle.is_empty() {
                sleep_remaining_time().await;
                continue;
            }

            for message in to_handle {
                let mut to_send = CreateMessage::new().content(message.message());

                if let Some(id) = message.poll_id {
                    let add_poll_result = add_poll(to_send, id);

                    if let Err(e) = add_poll_result {
                        error!(
                            "Failed to add poll to the scheduled message with id {}. Reason: {e}",
                            message.id()
                        );
                        continue;
                    }

                    to_send = add_poll_result.unwrap();
                }

                if let Some(locations) = &message.attachments {
                    if message.poll_id.is_some() {
                        error!("Cannot add attachments to a poll message. The attachments will be ignored.");
                    } else {
                        let add_attachments_result = add_attachments(to_send, locations).await;

                        if let Err(e) = add_attachments_result {
                            error!("Failed to add attachments to the scheduled message with id {}. Reason: {e}", message.id());
                            continue;
                        }

                        to_send = add_attachments_result.unwrap();
                    }
                }

                let mut quiz_data = None;
                if let Some(id) = message.quiz_id {
                    let quiz_data_result = QuizData::get_quiz_data(id);

                    if let Err(e) = quiz_data_result {
                        error!(
                            "Failed to get quiz data with id {id} for scheduled message with id {}. Reason: {e}",
                            message.id()
                        );
                        continue;
                    }
                    quiz_data = Some(quiz_data_result.unwrap())
                }

                let result = target_channel.1.send_message(&ctx, to_send).await;
                if let Err(e) = result {
                    info!("Failed to send scheduled message with id {}. This won't be set as completed. Reason: {e}", message.id());
                    continue;
                }

                info!("Scheduled message with id {} was sent", message.id());

                let sent_message = result.unwrap();

                let mut pin_message = pin_all;

                if let Some(to_pin) = message.to_pin {
                    pin_message = to_pin;
                }

                if pin_message {
                    let pin_result = sent_message.pin(&ctx).await;
                    if let Err(e) = pin_result {
                        error!("Failed to pin scheduled message with id {}. This message will be marked as completed regardless. Reason: {e}", message.id());
                    }
                }

                if let Some(data) = quiz_data {
                    set_ongoing_quiz(&ctx, data).await;
                }

                completed.add_new_completed(message.id());

                // Try to save the scheduled id as completed 3 times. If failed, exit the bot
                for num in 0..3 {
                    let save_result = completed.save_completed_scheduled();
                    if save_result.is_ok() {
                        break;
                    }
                    if num == 2 {
                        error!("Failed to save the scheduled message id as completed. This is a fatal error and the bot will be exited. \
                            The scheduled message was sent successfully but the id number could not be saved as completed. \
                            Before the next run, completed.json must be updated manually otherwise the same scheduled message will be sent again.\n\nReason: {}", save_result.unwrap_err());
                        std::process::exit(1)
                    }
                    sleep(Duration::from_secs(2)).await;
                }
            }

            sleep_remaining_time().await;
        }
    }
}
