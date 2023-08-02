use std::{
    fmt::Display,
    io::{self, stdin},
    time::SystemTime,
};

fn main() {
    let mut quiz = load_quiz();

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
        let mut correct_answer = "-1".to_string();
        for (i, answer) in q.answers.iter().enumerate() {
            if answer.is_correct {
                correct_answer = (i + 1).to_string();
            }
            println!("\t[{}]-{}", i + 1, answer.title);
        }
        println!("Your answer ?");
        let mut user_answer = String::new();
        stdin().read_line(&mut user_answer).expect("Can't read!");
        let user_answer = user_answer.trim();
        match user_answer {
            "1" | "2" | "3" | "4" => {}
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

    // println!("Soru : {} ({} puan)", quiz[0].title,quiz[0].point);
    // for (i, answer) in quiz[0].answers.iter().enumerate() {
    //     println!("\t[{}]-{}", i, answer.title);
    // }
}

struct Player {
    pub nick_name: String,
    pub enter_time: SystemTime,
    pub point: u32,
}
impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' - [{:?}]", self.nick_name, self.enter_time)
    }
}

struct Answer {
    pub title: String,
    pub is_correct: bool,
}

struct Question {
    pub title: String,
    pub answers: Vec<Answer>,
    pub point: u32,
}

fn load_quiz() -> Vec<Question> {
    vec![
        Question {
            title: "1 Byte bellekte kaç Bit yer kaplar?".to_string(),
            point: 4,
            answers: vec![
                Answer {
                    title: String::from("64 bit"),
                    is_correct: false,
                },
                Answer {
                    title: "32 bit".to_string(),
                    is_correct: false,
                },
                Answer {
                    title: "1024 bit".to_string(),
                    is_correct: false,
                },
                Answer {
                    title: "8 bit".to_string(),
                    is_correct: true,
                },
            ],
        },
        Question {
            title: "NoSQL'in açılımı nedir".to_string(),
            point: 3,
            answers: vec![
                Answer {
                    title: String::from("Not only SQL"),
                    is_correct: true,
                },
                Answer {
                    title: "Need only SQL".to_string(),
                    is_correct: false,
                },
                Answer {
                    title: "No SQL".to_string(),
                    is_correct: false,
                },
                Answer {
                    title: "Not Open SQL".to_string(),
                    is_correct: false,
                },
            ],
        },
        Question {
            title: "Rust dili ile aşağıdakilerden hangisi yanlıştır".to_string(),
            point: 8,
            answers: vec![
                Answer {
                    title: String::from("Değişkenler varsayılan olarak immutable'dır."),
                    is_correct: false,
                },
                Answer {
                    title: "Derleme zamanında Borrow-Checker mekanizması ile bellek kullanımları kontrol edilir.".to_string(),
                    is_correct: false,
                },
                Answer {
                    title: "RGC isimli Garbage Collector mekanizmasını kullanır.".to_string(),
                    is_correct: true,
                },
                Answer {
                    title: "Farklı dillerle entegrasyon için FFI kabiliyetine sahiptir.".to_string(),
                    is_correct: false,
                },
            ],
        },
        Question {
            title: "Rust dilinde bazı fonksiyonların arkasında ! işareti görüyoruz. vec!, println! gibi. Bu ne anlama gelir.".to_string(),
            point: 7,
            answers: vec![
                Answer {
                    title: String::from("Hataya sebep olabilecek bir fonksiyondur, dikkatli kullanılmalıdır."),
                    is_correct: false,
                },
                Answer {
                    title: "Bir makro olduğunu gösterir.".to_string(),
                    is_correct: true,
                },
                Answer {
                    title: "Fonksiyonu belirten parametre sayısı kadar çalıştırır.".to_string(),
                    is_correct: true,
                },
                Answer {
                    title: "Bir anlamı yoktur. Opsiyoneldir.".to_string(),
                    is_correct: false,
                },
            ],
        }
    ]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_first_question_loaded() {
        let quiz = load_quiz();
        assert!(quiz.len() > 0);
        assert_eq!(
            quiz[0].title,
            "1 Byte bellekte kaç Bit yer kaplar?".to_string()
        );
    }
}