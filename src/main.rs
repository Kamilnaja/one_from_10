mod answer;
mod create_questions;
mod question;
use crate::create_questions::create_questions;
use std::io;

fn main() {
    println!("Witamy w teleturnieju 1 z 10");
    let questions = create_questions();
    let mut correct_answers_count = 0;

    for (i, question) in questions.iter().enumerate() {
        println!("Pytanie {} - {}", i + 1, question.text);

        for (i, answer) in question.answers.iter().enumerate() {
            println!("{} - {}", i + 1, answer.text)
        }

        let mut user_answer = String::new();
        io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");
        let user_answer: i8 = user_answer.trim().parse().expect("Cannot format to int");
        let correct_answer_index = question.answers.iter().position(|x| x.is_correct);

        match correct_answer_index {
            Some(index) => {
                if (index + 1) as i8 == user_answer {
                    println!("Poprawna odpowiedź");
                    correct_answers_count += 1;
                } else {
                    println!(
                        "Niepoprawna odpowiedź. Poprawna odpowiedź to: {}",
                        index + 1
                    );
                }
            }
            None => println!("Brak poprawnej odpowiedzi w pytaniu"),
        }
    }
    println!("Podsumowanie");
    println!(
        "Odpowiedziałeś poprawnie na {} pytań z {}",
        correct_answers_count,
        questions.len()
    )
}
