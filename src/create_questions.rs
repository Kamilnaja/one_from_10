use crate::answer::Answer;
use crate::question::Question;

pub fn create_questions() -> Vec<Question> {
    vec![
        Question {
            text: String::from("Czy ryba to zwierze?"),
            answer: 1,
            answers: vec![Answer {
                text: String::from("Tak"),
                is_correct: true,
            }],
        },
        Question {
            text: String::from("Ile centymetrów zawiera jeden meter"),
            answer: 2,
            answers: vec![],
        },
        Question {
            text: String::from("Ile milimetrów zawiera jeden centymetr"),
            answer: 1,
            answers: vec![],
        },
        Question {
            text: String::from("Ile metrów zawiera jeden kilometr"),
            answer: 3,
            answers: vec![],
        },
        Question {
            text: String::from("Ile centymetrów zawiera jeden decymetr"),
            answer: 3,
            answers: vec![],
        },
    ]
}
