mod question;
use uuid::Uuid;

fn main() {
    let q1 = question::Question {
        id: Uuid::new_v4(),
        text: String::from("Question 1"),
        answer: String::from("Lorem ipsum dolor"),
    };
    println!("{} {} {}", q1.id, q1.text, q1.answer);
}
