use anyhow::{anyhow, Context, Error};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct QuizData {
    id: u32,
    answer: String,
    end_at: Option<DateTime<Utc>>,
    reply_with: Option<String>,
}

impl QuizData {
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn answer(&self) -> String {
        self.answer.clone()
    }

    pub fn end_at(&self) -> &Option<DateTime<Utc>> {
        &self.end_at
    }

    pub fn reply_with(&self) -> &Option<String> {
        &self.reply_with
    }

    pub fn get_all_quiz_data() -> Result<Vec<QuizData>, Error> {
        let mut file = File::open("config/quiz.json").context("Failed to open quiz.json")?;
        let mut json_string = String::new();

        file.read_to_string(&mut json_string)
            .context("Failed to read quiz.json")?;

        let result: Vec<QuizData> =
            serde_json::from_str(&json_string).context("Failed to parse quiz.json")?;
        Ok(result)
    }

    pub fn get_quiz_data(quiz_id: u32) -> Result<QuizData, Error> {
        let all_quizzes = QuizData::get_all_quiz_data().context("Failed to get all quiz data")?;

        for quiz in all_quizzes {
            if quiz.id == quiz_id {
                return Ok(quiz);
            }
        }
        Err(anyhow!("Quiz with id {} not found", quiz_id))
    }
}
