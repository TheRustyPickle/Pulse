use serde::Deserialize;
use serenity::model::id::ChannelId;
use std::error::Error;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct BotConfig {
    bot_token: String,
    target_guild: String,
    target_channel: String,
    #[serde(skip_deserializing)]
    target_channel_id: ChannelId,
}

impl BotConfig {
    pub fn get_config() -> Result<Self, Box<dyn Error>> {
        let mut file = File::open("config/bot_config.json")?;
        let mut json_string = String::new();

        file.read_to_string(&mut json_string)?;

        let result: BotConfig = serde_json::from_str(&json_string)?;
        Ok(result)
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

    pub fn get_target_channel_id(&self) -> ChannelId {
        self.target_channel_id
    }

    pub fn set_target_channel_id(&mut self, channel_id: ChannelId) {
        self.target_channel_id = channel_id
    }
}
