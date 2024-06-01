use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::Read};

#[derive(Deserialize, Clone)]
pub struct QuizData {
    id: u32,
    pub answer: String,
    pub started_at: DateTime<Utc>,
}

pub fn get_quiz_id(schedule_id: u32) -> Option<u32> {
    let schedule_quiz_ids = HashMap::from([(4, 1)]);

    schedule_quiz_ids.get(&(schedule_id as i32)).copied()
}

pub fn get_all_quiz_data() -> Vec<QuizData> {
    let mut file = File::open("quizzes.json").unwrap();
    let mut json_string = String::new();

    file.read_to_string(&mut json_string).unwrap();

    serde_json::from_str(&json_string).unwrap()
}

pub fn get_quiz_data(quiz_id: u32) -> QuizData {
    let all_quizzes = get_all_quiz_data();

    for quiz in all_quizzes {
        if quiz.id == quiz_id {
            return quiz;
        }
    }
    unreachable!()
}
