mod model;
mod ware_house;

use crate::model::Player;
use crate::ware_house::WareHouse;
use std::{
    io::{self, stdin},
    time::SystemTime,
};

fn main() {
    let mut quiz = WareHouse::load_quiz();

    println!("Nick name? ");
    let mut nick_name = String::new();
    let reader = io::stdin();
    reader.read_line(&mut nick_name).expect("Can't read!"); // Nasıl hata yaptırtabiliriz?
    let nick_name = nick_name.trim().to_string();
    let mut player = Player {
        nick_name,
        enter_time: SystemTime::now(), // bunun yerine chrono küfesini kullanıp Local::now() ile ilerlemek lazım.
        point: 0,
    };
    println!("{}", player);

    while let Some(q) = quiz.pop() {
        println!("{}", q.title);
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
            player.point += q.point;
        }
    }

    println!("Your total score is {}", player.point);
}

#[cfg(test)]
mod test {
    use crate::ware_house::*;

    #[test]
    fn should_first_question_loaded() {
        let quiz = WareHouse::load_quiz();
        assert!(quiz.len() > 0);
        assert_eq!(
            quiz[0].title,
            "1 Byte bellekte kaç Bit yer kaplar?".to_string()
        );
    }
}
