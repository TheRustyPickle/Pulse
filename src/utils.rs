use anyhow::{anyhow, Error};
use chrono::{Timelike, Utc};
use serenity::builder::{CreateAttachment, CreateMessage, CreatePoll, CreatePollAnswer};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::path::Path;
use tokio::time::{sleep, Duration};
use tracing::info;

use crate::config::PollData;

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

/// Add a poll to a discord message
pub fn add_poll(message: CreateMessage, id: u32) -> Result<CreateMessage, Error> {
    let poll_data = PollData::get_poll_data(id);

    if let Err(e) = poll_data {
        return Err(anyhow!("Failed to read poll message data. Reason: {e}"));
    }

    let poll_data = poll_data.unwrap();

    let mut poll_questions = Vec::new();

    for question in poll_data.questions() {
        poll_questions.push(CreatePollAnswer::new().text(question));
    }

    let poll = CreatePoll::new()
        .question(poll_data.message())
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
pub async fn sleep_remaining_time() {
    let now = Utc::now();
    let seconds_remaining = (59 - now.second() + 1) as u64;
    sleep(Duration::from_secs(seconds_remaining)).await;
}
