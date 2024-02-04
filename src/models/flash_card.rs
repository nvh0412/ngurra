use std::{collections::HashMap, time::SystemTime};

#[derive(Debug)]
pub struct FlashCard {
    question: String,
    answer: String,
    creation_date: SystemTime,
    last_studied_date: SystemTime,
    performance_metrics: HashMap<String, i32>
}

impl FlashCard {
    pub fn new(question: &str, answer: &str) -> FlashCard {
        FlashCard {
            question: question.to_string(),
            answer: answer.to_string(),
            creation_date: SystemTime::now(),
            last_studied_date: SystemTime::now(),
            performance_metrics: HashMap::new()
        }
    }
}
