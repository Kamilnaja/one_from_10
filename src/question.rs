use uuid::Uuid;

pub struct Question {
    pub id: Uuid,
    pub text: String,
    pub answer: String,
}
