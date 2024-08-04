pub mod bot;
pub mod config;
pub mod utils;

use bot::Handler;
use config::{BotConfig, QuizData};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::error;

/// Saves a quiz data to track whether a quiz is ongoing
pub struct OngoingQuiz;

impl TypeMapKey for OngoingQuiz {
    type Value = Arc<Mutex<Option<QuizData>>>;
}

pub struct ThreadStarted;

impl TypeMapKey for ThreadStarted {
    type Value = AtomicBool;
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let bot_config = BotConfig::get_config();

    if let Err(e) = bot_config {
        error!("Failed to read the bot config file. Error: {e}");
        std::process::exit(1)
    }

    let config = bot_config.unwrap();

    let token = config.get_token();
    let intent = GatewayIntents::default().union(GatewayIntents::MESSAGE_CONTENT);

    let mut client = Client::builder(token, intent)
        .event_handler(Handler)
        .await
        .expect("failed to create the client");

    {
        let mut data = client.data.write().await;
        data.insert::<OngoingQuiz>(Arc::new(Mutex::new(None)));
    }

    {
        let mut data = client.data.write().await;
        data.insert::<ThreadStarted>(AtomicBool::new(false));
    }

    if let Err(e) = client.start().await {
        error!("Client error: {e}");
    }
}
