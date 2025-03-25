#[path = "answer.rs"]
mod answer;
use answer::Answer;
pub struct Question {
    pub text: String,
    pub answer: u8,
    pub answers: Vec<Answer>,
}
