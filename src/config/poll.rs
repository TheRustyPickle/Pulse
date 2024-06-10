use anyhow::{anyhow, Context, Error};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct PollData {
    id: u32,
    question: String,
    answers: Vec<String>,
    duration_minutes: Option<u64>,
    multi_answer: Option<bool>,
}

impl PollData {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn question(&self) -> String {
        self.question.clone()
    }

    pub fn answers(&self) -> &Vec<String> {
        &self.answers
    }

    pub fn duration_minutes(&self) -> u64 {
        if let Some(min) = self.duration_minutes {
            min
        } else {
            1440
        }
    }

    pub fn multi_answer(&self) -> bool {
        if let Some(multi) = self.multi_answer {
            multi
        } else {
            false
        }
    }

    pub fn get_all_polls() -> Result<Vec<PollData>, Error> {
        let mut file = File::open("config/poll.json").context("Failed to open poll.json file")?;
        let mut json_string = String::new();

        file.read_to_string(&mut json_string)
            .context("Failed to read poll.json")?;
        let result: Vec<PollData> =
            serde_json::from_str(&json_string).context("Failed to parse poll.json file")?;
        Ok(result)
    }

    pub fn get_poll_data(id: u32) -> Result<PollData, Error> {
        let all_poll = PollData::get_all_polls()?;
        for poll in all_poll {
            if poll.id == id {
                return Ok(poll);
            }
        }
        Err(anyhow!("Poll with id {} not found", id))
    }
}
