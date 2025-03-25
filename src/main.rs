struct Question {
    id: u32,
    text: String,
    answer: String,
}

fn main() {
    let q1 = Question {
        id: 1,
        text: String::from("Question 1"),
        answer: String::from("Lorem ipsum dolor"),
    };
    println!("{} {} {}", q1.id, q1.text, q1.answer);
}
