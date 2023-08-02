use crate::model::*;

pub struct WareHouse;

impl WareHouse {
    pub fn load_quiz() -> Vec<Question> {
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
}
