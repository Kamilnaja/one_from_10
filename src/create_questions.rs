#[path = "question.rs"]
mod question; // Declares the question module from question.rs
use question::Question;
#[path = "answer.rs"]
mod answer;

pub fn create_questions() -> Vec<Question> {
    vec![
        question::Question {
            text: String::from("Czy ryba to zwierze?"),
            answer: 1,
            answers: vec![],
        },
        question::Question {
            text: String::from("Ile centymetrów zawiera jeden meter"),
            answer: 2,
            answers: vec![],
        },
        question::Question {
            text: String::from("Ile milimetrów zawiera jeden centymetr"),
            answer: 1,
            answers: vec![],
        },
        question::Question {
            text: String::from("Ile metrów zawiera jeden kilometr"),
            answer: 3,
            answers: vec![],
        },
        question::Question {
            text: String::from("Ile centymetrów zawiera jeden decymetr"),
            answer: 3,
            answers: vec![],
        },
    ]
}
