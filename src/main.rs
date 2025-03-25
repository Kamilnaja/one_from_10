mod create_questions;
fn main() {
    println!("Witamy w teleturnieju 1 z 10");
    let questions = create_questions::create_questions();
    for question in questions {
        println!("{}", question.text);
    }
}
