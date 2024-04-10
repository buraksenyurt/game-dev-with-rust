#[derive(Default, Clone)]
pub struct Level {
    pub id: u32,
    pub title: String,
    pub map: String,
    pub question: String,
    pub question_answer: String,
}

pub struct LevelManager {
    maps: Vec<Level>,
}

impl LevelManager {
    pub fn init() -> Self {
        let mut maps = vec![];
        let first_level = Level {
            id: 0,
            title: "Giriş Seviyesi".to_string(),
            map: "wwwwwwwwww\
                \nwwtwttttte\
                \nwtssstwttw\
                \npsswtqtwtw\
                \nwwwwtwwwsw\
                \nwwwttttstw\
                \nwwwwwttstw\
                \nwwwwwwwwww"
                .to_string(),
            question:"Bir çemberin çevresinin çapına oranı PI sayısı ile ifade edilir. Doğru mu yanlış mı?".to_string(),
            question_answer:"D".to_string()
        };

        maps.push(first_level);
        Self { maps }
    }

    pub fn get_level(&self, index: u32) -> Option<Level> {
        Some(self.maps[index as usize].clone())
    }
}
