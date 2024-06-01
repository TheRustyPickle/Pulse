use anyhow::{anyhow, Error};
use chrono::{Timelike, Utc};
use serenity::all::CreateAttachment;
use serenity::async_trait;
use serenity::builder::{CreateMessage, CreatePoll, CreatePollAnswer};
use serenity::model::gateway::Ready;
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::path::Path;
// use std::sync::Arc;
use tokio::spawn;
use tokio::time::{sleep, Duration};
use tracing::{error, info};

use crate::config::BotConfig;
use crate::poll::get_poll_data;
// use crate::quiz::{get_quiz_data, get_quiz_id};
use crate::schedule::{CompletedScheduled, ScheduledMessage};
// use crate::QuizStarted;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _ready: Ready) {
        spawn(async move {
            Self::start_ticking(ctx).await;
        });

        info!("The bot is ready");
    }

    // async fn message(&self, ctx: Context, new_message: Message) {
    //     if new_message.channel_id != TARGET_CHANNEL_ID {
    //         info!("Not the correct channel");
    //         return;
    //     }
    //     {
    //         let data_read = ctx.data.read().await;
    //         let data = data_read.get::<QuizStarted>().unwrap();
    //         let quiz_start = data.lock().await;
    //
    //         if quiz_start.is_none() {
    //             info!("Quiz hasn't started yet");
    //             return;
    //         }
    //
    //         let quiz_answer = quiz_start.clone().unwrap().answer;
    //
    //         let message_content = new_message
    //             .content
    //             .to_lowercase()
    //             .replace(['\n', '-', ',', '.', '!', '?', ':', '-'], " ");
    //
    //         let split_content = message_content.split_whitespace().collect::<Vec<_>>();
    //
    //         if split_content.contains(&quiz_answer.as_str()) {
    //             let winner = new_message.member(&ctx.http).await.unwrap();
    //             new_message
    //                 .reply(
    //                     &ctx.http(),
    //                     format!("{} You are the winner!", winner.mention()),
    //                 )
    //                 .await
    //                 .unwrap();
    //             {
    //                 let mut data = ctx.data.write().await;
    //                 data.insert::<QuizStarted>(Arc::new(Mutex::new(None)));
    //             }
    //         }
    //
    //         info!("Message gotten: {}", new_message.content);
    //     };
    // }
}

impl Handler {
    async fn start_ticking(ctx: Context) {
        let bot_config = BotConfig::get_config();

        if let Err(e) = bot_config {
            error!("Error reading the bot config. Error: {e}");
            std::process::exit(1);
        }
        let config = bot_config.unwrap();

        // Wait 5 seconds for the cache to load
        sleep(Duration::from_secs(5)).await;

        let target_guild_name = config.get_target_guild();
        let target_channel_name = config.get_target_guild();

        let mut target_channel = None;
        let mut target_guild = get_target_guild(&ctx, &target_guild_name).await;

        // keep looping until the target guild is found
        while target_guild.is_none() {
            error!("Target guild not found. Trying again in 60 seconds");
            sleep(Duration::from_secs(60)).await;
            target_guild = get_target_guild(&ctx, &target_guild_name).await;
        }

        let target_guild = target_guild.unwrap();

        // keep looping until the target channel is found
        while target_channel.is_none() {
            for target in &target_guild.channels {
                if target.1.name() == target_channel_name {
                    target_channel = Some(target);
                    break;
                }
            }
            if target_channel.is_none() {
                error!("Failed to get target channel in the guild. Trying again in 60 seconds");
                sleep(Duration::from_secs(60)).await;
                continue;
            }
        }

        let target_channel = target_channel.unwrap();
        info!("Target channel found");
        info!("Starting scheduling.");

        loop {
            // If anything fails during this loop, we sleep till the current minute ends and try
            // again the next minute

            let schedule_data = ScheduledMessage::get_all_scheduled_messages();
            let completed_data = CompletedScheduled::get_completed_scheduled();

            if let Err(e) = &schedule_data {
                error!("Failed to read scheduled message data. Reason: {e}");
                sleep_remaining_time().await;
                continue;
            }

            if let Err(e) = &completed_data {
                error!("Failed to read scheduled message data. Reason: {e}");
                sleep_remaining_time().await;
                continue;
            }

            let scheduled = schedule_data.unwrap();
            let mut completed = completed_data.unwrap();
            let mut to_handle = Vec::new();

            let now = Utc::now();

            for message in scheduled.iter() {
                if !completed.completed.contains(&message.id) && now >= message.scheduled_at {
                    to_handle.push(message)
                }
            }

            if to_handle.is_empty() {
                sleep_remaining_time().await;
                continue;
            }

            for message in to_handle {
                let mut to_send = CreateMessage::new().content(message.message.clone());

                if let Some(id) = message.poll_id {
                    let add_poll_result = add_poll(to_send, id).await;

                    if let Err(e) = add_poll_result {
                        error!("Failed to add poll to the message. Reason: {e}");
                        continue;
                    }

                    to_send = add_poll_result.unwrap();
                }

                if let Some(locations) = &message.attachments {
                    let add_attachments_result = add_attachments(to_send, locations).await;

                    if let Err(e) = add_attachments_result {
                        error!("Failed to add attachments to the message. Reason: {e}");
                        continue;
                    }

                    to_send = add_attachments_result.unwrap();
                }

                let result = target_channel.1.send_message(&ctx.http, to_send).await;
                if let Err(e) = result {
                    info!("Failed to send the message. Reason: {e}");
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }

                info!("Schedule message with id {} was sent", message.id);

                let sent_message = result.unwrap();

                // let pin_result = sent_message.pin(&ctx.http).await;
                // if let Err(e) = pin_result {
                //     error!("Failed to pin the message. Reason: {e}");
                // }
                // if let Some(id) = get_quiz_id(message.id) {
                //     let quiz = get_quiz_data(id);
                //     let mut data = ctx.data.write().await;
                //     data.insert::<QuizStarted>(Arc::new(Mutex::new(Some(quiz))))
                // }
                //
                completed.add_new_completed(message.id);

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

            sleep_remaining_time().await
        }
    }
}

/// Try to find the target Guild in the bot guild list
pub async fn get_target_guild(ctx: &Context, target_guild: &str) -> Option<Guild> {
    info!("Trying to find {target_guild}");

    let guilds = ctx.cache.guilds();

    for guild in guilds {
        if let Some(g) = guild.to_guild_cached(&ctx.cache) {
            if g.name == target_guild {
                info!("Found the target guild");
                return Some(g.to_owned());
            }
        }
    }
    None
}

/// Add a poll to a discord message
pub async fn add_poll(message: CreateMessage, id: u32) -> Result<CreateMessage, Error> {
    let poll_data = get_poll_data(id);

    if let Err(e) = poll_data {
        return Err(anyhow!("Failed to read poll message data. Reason: {e}"));
    }

    let poll_data = poll_data.unwrap();

    let mut poll_questions = Vec::new();

    for question in poll_data.questions {
        poll_questions.push(CreatePollAnswer::new().text(question))
    }

    let poll = CreatePoll::new()
        .question(poll_data.message)
        .answers(poll_questions)
        .duration(std::time::Duration::from_secs(60 * 60 * 24));

    Ok(message.poll(poll))
}

/// Add attachments to a discord message
pub async fn add_attachments(
    mut message: CreateMessage,
    locations: &Vec<String>,
) -> Result<CreateMessage, Error> {
    for location in locations {
        let attachment_path = Path::new(location);
        let attachment = CreateAttachment::path(attachment_path).await;
        match attachment {
            Ok(att) => message = message.add_file(att),
            Err(e) => {
                return Err(anyhow!(
                "Failed to create attachment for the message from location {location}. Reason: {e}"
            ))
            }
        }
    }

    Ok(message)
}

/// Sleep for the remaining seconds in a minute
async fn sleep_remaining_time() {
    let now = Utc::now();
    let seconds_remaining = (59 - now.second() + 1) as u64;
    sleep(Duration::from_secs(seconds_remaining)).await;
}
