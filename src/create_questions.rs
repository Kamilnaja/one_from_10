use crate::question;
use crate::question::Question;
use uuid::Uuid;

pub fn create_questions() -> Vec<Question> {
    vec![
        question::Question {
            id: Uuid::new_v4(),
            text: String::from("Orka na ..."),
            answer: String::from("Ugurze "),
        },
        question::Question {
            id: Uuid::new_v4(),
            text: String::from("Ile centymetrów zawiera jeden meter"),
            answer: String::from("100"),
        },
        question::Question {
            id: Uuid::new_v4(),
            text: String::from("Ile milimetrów zawiera jeden centymetr"),
            answer: String::from("10"),
        },
        question::Question {
            id: Uuid::new_v4(),
            text: String::from("Ile metrów zawiera jeden kilometr"),
            answer: String::from("1000"),
        },
        question::Question {
            id: Uuid::new_v4(),
            text: String::from("Ile centymetrów zawiera jeden decymetr"),
            answer: String::from("10"),
        },
    ]
}
