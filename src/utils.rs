use anyhow::{Error, anyhow};
use chrono::{Timelike, Utc};
use serenity::builder::{CreateAttachment, CreateMessage, CreatePoll, CreatePollAnswer};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::path::Path;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::info;
use std::time;

use crate::config::{BotConfig, PollData, QuizData};
use crate::{OngoingQuiz, SavedBotConfig};

/// Try to find the target Guild in the bot guild list
pub fn get_target_guild(ctx: &Context, target_guild: &str) -> Option<Guild> {
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

pub fn get_target_channel(guild: Guild, channel_name: &str) -> Option<(ChannelId, GuildChannel)> {
    for (channel_id, channel) in guild.channels {
        if channel.name() == channel_name {
            return Some((channel_id, channel))
        }
    }
    None
}

/// Add a poll to a discord message
pub fn add_poll(message: CreateMessage, id: u32) -> Result<CreateMessage, Error> {
    let poll_data = PollData::get_poll_data(id)?;

    let mut poll_answers= Vec::new();

    for question in poll_data.answers() {
        poll_answers.push(CreatePollAnswer::new().text(question));
    }

    if poll_answers.len() < 2 {
        return Err(anyhow!("Total answer for this poll is less than 2 which is invalid"))
    }

    // TODO: take an additional param to set duration
    // TODO: take an additional param to set whether multi answer
    let poll = CreatePoll::new()
        .question(poll_data.question())
        .answers(poll_answers)
        .duration(time::Duration::from_secs(60 * 60 * 24));

    Ok(message.poll(poll))
}

/// Add a file path as attachments to a discord message
pub async fn add_attachments(
    mut message: CreateMessage,
    locations: &Vec<String>,
) -> Result<CreateMessage, Error> {
    for location in locations {
        let attachment_path = Path::new(location);
        let attachment = CreateAttachment::path(attachment_path).await?;
        message = message.add_file(attachment);
    }

    Ok(message)
}

/// Sleep for the remaining seconds in a minute
pub async fn sleep_remaining_time() {
    let now = Utc::now();
    // seconds returns value from 0-59, 1 second extra added so if it's the last second, it sleeps
    // for 1 second at least ensuring it has went to the next minute
    let seconds_remaining = (60 - now.second()) as u64;
    sleep(Duration::from_secs(seconds_remaining)).await;
}

/// Saves bot config with global access
pub async fn save_bot_config(ctx: &Context, config: BotConfig) {
    let mut data = ctx.data.write().await;
    data.insert::<SavedBotConfig>(Arc::new(RwLock::new(config)));
}

/// Gets the target channel id from the saved bot config
pub async fn get_target_channel_id(ctx: &Context) -> ChannelId {
    let data_read = ctx.data.read().await;
    let data = data_read.get::<SavedBotConfig>().unwrap();
    let config = data.read().await;
    config.get_target_channel_id()
}

/// Saves a `QuizData` as ongoing with global access
pub async fn set_ongoing_quiz(ctx: &Context, quiz_data: QuizData) {
    let mut data = ctx.data.write().await;
    data.insert::<OngoingQuiz>(Arc::new(Mutex::new(Some(quiz_data))));
}

/// Removes the value saved in place of `OngoingQuiz`
pub async fn remove_ongoing_quiz(ctx: &Context) {
    let mut data = ctx.data.write().await;
    data.insert::<OngoingQuiz>(Arc::new(Mutex::new(None)));
}

/// Whether the content contains the target answer in the exact same order
pub fn contains_answer(content: Vec<&str>, answer: Vec<&str>) -> bool {
    if answer.len() == 1 && content.contains(&answer[0]) {
        return true;
    }

    if answer.len() > content.len() {
        return false;
    }

    content
        .windows(answer.len())
        .any(|window| window == answer.as_slice())
}
