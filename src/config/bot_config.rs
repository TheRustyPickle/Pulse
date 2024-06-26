use anyhow::Context;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct BotConfig {
    bot_token: String,
    target_guild: String,
    target_channel: String,
    pin_all: Option<bool>,
}

impl BotConfig {
    pub fn get_config() -> Result<Self, Box<dyn Error>> {
        let mut file =
            File::open("config/bot_config.json").context("Failed to open bot_config.json")?;
        let mut json_string = String::new();

        file.read_to_string(&mut json_string)
            .context("Failed to read bot_config.json")?;

        let result: BotConfig =
            serde_json::from_str(&json_string).context("Failed to parse bot_config.json")?;
        Ok(result)
    }

    pub fn pin_all(&self) -> bool {
        self.pin_all.unwrap_or_default()
    }

    pub fn get_token(&self) -> String {
        self.bot_token.clone()
    }

    pub fn get_target_guild(&self) -> String {
        self.target_guild.clone()
    }

    pub fn get_target_channel(&self) -> String {
        self.target_channel.clone()
    }
}
