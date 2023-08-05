mod model;
mod view;
mod ware_house;

use crate::model::{Player, Score};
use crate::view::Terminal;
use crate::ware_house::WareHouse;
use chrono::Local;
use std::io::stdin;
use std::path::Path;

fn main() {
    let mut quiz = WareHouse::load_quiz(Path::new("./questions.json")).expect("System problem.");
    Terminal::print_intro();
    println!("Please give me your name");
    let mut nick_name = String::new();
    stdin().read_line(&mut nick_name).expect("Can't read!"); // Nasıl hata yaptırtabiliriz?
    let nick_name = nick_name.trim().to_string();
    println!("Hello {} :)", nick_name);

    let mut player = Player {
        nick_name,
        enter_time: Local::now().time(), // bunun yerine chrono küfesini kullanıp Local::now() ile ilerlemek lazım.
        score: Score::default(),
    };
    println!("{}", player);

    while let Some(q) = quiz.pop() {
        println!("\nQ : {} [{}] Point", q.title, q.point);
        let mut correct_answer: usize = 0;
        for (i, answer) in q.answers.iter().enumerate() {
            if answer.is_correct {
                correct_answer = i + 1;
            }
            println!("\t[{}]-{}", i + 1, answer.title);
        }
        println!("Your answer ?");
        let mut user_answer = String::new();
        stdin().read_line(&mut user_answer).expect("Can't read!");
        let user_answer = user_answer.trim().parse::<usize>().expect("isn't numeric");
        match user_answer {
            1..=4 => {}
            _ => {
                println!("Please write valid choice (1,2,3,4)");
                quiz.push(q);
                continue;
            }
        }
        if user_answer == correct_answer {
            player.score.total_point += q.point;
            player.score.correct += 1;
        } else {
            player.score.wrong += 1;
        }
    }

    println!("Game finished.");
    println!("{}", player.score);
}

#[cfg(test)]
mod test {
    use crate::ware_house::*;
    use std::path::Path;

    #[test]
    fn should_first_question_loaded() {
        let quiz = WareHouse::load_quiz(Path::new("./questions.json")).unwrap();
        assert!(quiz.len() > 0);
        assert_eq!(
            quiz[0].title,
            "1 Byte bellekte kaç Bit yer kaplar?".to_string()
        );
    }
}
