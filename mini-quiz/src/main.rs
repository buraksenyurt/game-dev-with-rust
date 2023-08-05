mod model;
mod view;
mod ware_house;

use crate::model::{Player, Score};
use crate::view::Terminal;
use crate::ware_house::WareHouse;
use chrono::Local;
use colorized::{Color, Colors};
use std::io::stdin;
use std::path::Path;

fn main() {
    let mut quiz = WareHouse::load_quiz(Path::new("./questions.json")).expect("System problem.");
    Terminal::print_intro();
    println!("\nPlease give me your name\n");
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
        let correct_answer = Terminal::display_question(&q);
        let mut user_answer = String::new();
        stdin().read_line(&mut user_answer).expect("Can't read!");
        let user_answer = user_answer.trim().parse::<usize>().expect("isn't numeric");
        match user_answer {
            1..=4 => {}
            _ => {
                let text = "Please write valid choice (1,2,3,4)".color(Colors::RedBg);
                println!("{}", text);
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

    println!("\nGame finished.");
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
