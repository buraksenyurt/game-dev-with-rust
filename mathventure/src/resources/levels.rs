#[derive(Default, Clone)]
pub struct Level {
    pub id: u32,
    pub title: String,
    pub map: String, //TODO@Burak Convert to Map data structure
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
            question:"Bir çemberin çevresinin çapına oranı PI sayısı ile ifade edilir. Doğru mu yanlış mı?".to_string(),
            question_answer:"D".to_string()
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
            question:"Her dik üçgenin hipotenüsü, diğer iki kenarının uzunluklarının toplamına eşittir. Doğru mu Yanlış mı?".to_string(),
            question_answer:"Y".to_string()
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
            question:"Bir üçgenin iç açılarının toplamı 180 derecedir. Doğru mu Yanlış mı?".to_string(),
            question_answer:"D".to_string()
        };
        maps.push(third_level);

        Self { maps }
    }

    pub fn get_level(&self, index: u32) -> Option<Level> {
        Some(self.maps[index as usize].clone())
    }
}
