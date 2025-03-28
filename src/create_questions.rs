use crate::answer::build_answer;
use crate::question::build_question;
use crate::question::Question;

pub fn create_questions() -> Vec<Question> {
    vec![
        build_question(
            String::from("Czy ryba to zwierze?"),
            vec![
                build_answer(String::from("Tak"), true),
                build_answer(String::from("Nie"), false),
            ],
        ),
        build_question(
            String::from("Jakiego koloru jest trawa"),
            vec![
                build_answer(String::from("Zielona"), true),
                build_answer(String::from("Czerwona"), false),
            ],
        ),
        build_question(
            String::from("Ile nóg ma pająk"),
            vec![
                build_answer(String::from("4"), false),
                build_answer(String::from("8"), true),
            ],
        ),
    ]
}
