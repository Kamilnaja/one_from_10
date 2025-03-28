use crate::answer::Answer;
pub struct Question {
    pub text: String,
    pub answers: Vec<Answer>,
}

pub fn build_question(text: String, answers: Vec<Answer>) -> Question {
    Question { text, answers }
}
