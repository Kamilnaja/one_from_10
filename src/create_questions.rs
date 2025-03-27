use crate::answer::Answer;
use crate::question::Question;

pub fn create_questions() -> Vec<Question> {
    vec![
        Question {
            text: String::from("Czy ryba to zwierze?"),
            answers: vec![
                Answer {
                    text: String::from("Tak"),
                    is_correct: true,
                },
                Answer {
                    text: String::from("Nie"),
                    is_correct: false,
                },
            ],
        },
        Question {
            text: String::from("Jakiego koloru jest trawa"),
            answers: vec![
                Answer {
                    text: String::from("Zielona"),
                    is_correct: true,
                },
                Answer {
                    text: String::from("Czerwona"),
                    is_correct: false,
                },
            ],
        },
        Question {
            text: String::from("Ile nóg ma pająk"),
            answers: vec![
                Answer {
                    text: String::from("4"),
                    is_correct: false,
                },
                Answer {
                    text: String::from("8"),
                    is_correct: true,
                },
            ],
        },
    ]
}
