use crate::entity::Question;

#[derive(Default, Clone)]
pub struct Level {
    pub id: u32,
    pub title: String,
    pub map: String,
    pub question: Question,
}

pub struct LevelManager {
    maps: Vec<Level>,
    pub max_level_count: u32,
}

impl LevelManager {
    pub fn init() -> Self {
        let mut maps = vec![];

        let first_level = Level {
            id: 0,
            title: "I. Seviye".to_string(),
            map: "wwwwwwwwww\
                \nwwtwttttte\
                \nwtssstwttw\
                \npsswtqtwtw\
                \nwwwwtwwwsw\
                \nwwwttttstw\
                \nwwwwwttstw\
                \nwwwwwwwwww"
                .to_string(),
            question:Question::new("Bir çemberin çevresinin çapına oranı PI sayısı ile ifade edilir. Doğru mu yanlış mı?".to_string(),
            "D".to_string())
        };
        maps.push(first_level);

        let second_level = Level {
            id: 1,
            title: "II. Seviye".to_string(),
            map: "wwwwwwwwww\
                \ntttttttwww\
                \nstssstttww\
                \npsssqtttte\
                \nssststtwww\
                \nttttstwwww\
                \nttsstwwwww\
                \nwwwtwwwwww"
                .to_string(),
            question: Question::new("Her dik üçgenin hipotenüsü, diğer iki kenarının uzunluklarının toplamına eşittir. Doğru mu Yanlış mı?".to_string(),
            "Y".to_string())
        };
        maps.push(second_level);

        let third_level = Level {
            id: 2,
            title: "III. Seviye".to_string(),
            map: "wwwwwwwwww\
                \nwtttqststw\
                \nwtsssttttw\
                \nwpsssttttw\
                \nwsststtwtw\
                \nwtttstwwtw\
                \nwtssqtwtte\
                \nwwwwwwwwww"
                .to_string(),
            question: Question::new(
                "Bir üçgenin iç açılarının toplamı 180 derecedir. Doğru mu Yanlış mı?".to_string(),
                "D".to_string(),
            ),
        };
        maps.push(third_level);
        let max_level_count = maps.len() as u32 - 1;

        Self {
            maps,
            max_level_count,
        }
    }

    pub fn get_level(&self, index: u32) -> Option<Level> {
        Some(self.maps[index as usize].clone())
    }
}
