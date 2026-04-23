use crate::model::Question;
use colorized::{Color, Colors};

pub struct Terminal;

impl Terminal {
    pub fn print_intro() {
        let line = "*************************";
        let empty_line = "**                     **";
        let lines = vec![
            line,
            empty_line,
            empty_line,
            "**      MİNİ QUİZ      **",
            "**   Developed By BSŞ  **",
            empty_line,
            "**        2023         **",
            empty_line,
            empty_line,
            line,
        ];
        for line in lines {
            println!("{}", line.color(Colors::CyanFg));
        }
    }

    pub fn display_question(q: &Question) -> usize {
        let mut correct_answer_id = 0;
        println!("\n{}", q.to_string().color(Colors::YellowFg));
        for (i, answer) in q.answers.iter().enumerate() {
            if answer.is_correct {
                correct_answer_id = i + 1;
            }
            let text = format!("\t[{}]-{}", i + 1, answer.title);
            println!("{}", text.color(Colors::MagentaFg));
        }
        let text = "What is your answer ?".color(Colors::BrightBlueFg);
        println!("{}", text);
        correct_answer_id
    }
}
