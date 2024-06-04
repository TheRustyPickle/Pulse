use anyhow::{Context, Error};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Deserialize, Clone)]
pub struct ScheduledMessage {
    id: u32,
    message: String,
    pub attachments: Option<Vec<String>>,
    scheduled_at: DateTime<Utc>,
    pub poll_id: Option<u32>,
    pub quiz_id: Option<u32>,
    pub to_pin: Option<bool>,
}

impl ScheduledMessage {
    pub fn get_all_scheduled_messages() -> Result<Vec<Self>, Error> {
        let mut file =
            File::open("config/schedule.json").context("Failed to open schedule.json file")?;
        let mut json_string = String::new();

        file.read_to_string(&mut json_string)
            .context("Failed to read schedule.json").context("Failed to read schedule.json")?;

        let result: Vec<ScheduledMessage> =
            serde_json::from_str(&json_string).context("Failed to parse schedule.json file")?;
        Ok(result)
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn scheduled_at(&self) -> DateTime<Utc> {
        self.scheduled_at
    }
}

#[derive(Deserialize, Serialize)]
pub struct CompletedScheduled {
    pub completed: HashSet<u32>,
}

impl CompletedScheduled {
    pub fn add_new_completed(&mut self, id: u32) {
        self.completed.insert(id);
    }

    pub fn get_completed_scheduled() -> Result<CompletedScheduled, Error> {
        let mut file = File::open("config/completed.json").context("Failed to open completed.json")?;
        let mut json_string = String::new();

        file.read_to_string(&mut json_string).context("Failed to read completed.json")?;

        let result: CompletedScheduled =
            serde_json::from_str(&json_string).context("Failed to parse completed.json file")?;
        Ok(result)
    }

    pub fn save_completed_scheduled(&self) -> Result<(), Error> {
        let serialized_data = serde_json::to_string(self).context("Failed to serialize data")?;

        let mut file = File::create("config/completed.json").context("Failed to create completed.json")?;

        file.write_all(serialized_data.as_bytes()).context("Failed to write to completed.json")?;
        Ok(())
    }
}
