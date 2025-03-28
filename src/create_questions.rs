use crate::answer::Answer;
use crate::question::build_question;
use crate::question::Question;

pub fn create_questions() -> Vec<Question> {
    vec![
        build_question(
            String::from("Czy ryba to zwierze?"),
            vec![
                Answer {
                    text: String::from("Tak"),
                    is_correct: true,
                },
                Answer {
                    text: String::from("Nie"),
                    is_correct: false,
                },
            ],
        ),
        build_question(
            String::from("Jakiego koloru jest trawa"),
            vec![
                Answer {
                    text: String::from("Zielona"),
                    is_correct: true,
                },
                Answer {
                    text: String::from("Czerwona"),
                    is_correct: false,
                },
            ],
        ),
        build_question(
            String::from("Ile nóg ma pająk"),
            vec![
                Answer {
                    text: String::from("4"),
                    is_correct: false,
                },
                Answer {
                    text: String::from("8"),
                    is_correct: true,
                },
            ],
        ),
    ]
}
