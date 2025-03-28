pub struct Answer {
    pub text: String,
    pub is_correct: bool,
}

pub fn build_answer(text: String, is_correct: bool) -> Answer {
    Answer { text, is_correct }
}
