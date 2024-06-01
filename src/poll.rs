use anyhow::{anyhow, Context, Error};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct PollData {
    pub id: u32,
    pub message: String,
    pub questions: Vec<String>,
}

pub fn get_all_polls() -> Result<Vec<PollData>, Error> {
    let mut file = File::open("poll.json")?;
    let mut json_string = String::new();

    file.read_to_string(&mut json_string)?;
    let result: Vec<PollData> =
        serde_json::from_str(&json_string).context("Failed to parse schedule.json file")?;
    Ok(result)
}

pub fn get_poll_data(id: u32) -> Result<PollData, Error> {
    let all_poll = get_all_polls()?;
    for poll in all_poll {
        if poll.id == id {
            return Ok(poll);
        }
    }
    Err(anyhow!("Poll with id {} not found", id))
}
