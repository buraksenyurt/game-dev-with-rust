use crate::model::*;
use rand::prelude::SliceRandom;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

pub struct WareHouse;

impl WareHouse {
    pub fn load_quiz<P: AsRef<Path>>(path: P) -> Result<Vec<Question>, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut questions: Vec<Question> = serde_json::from_reader(reader)?;

        let mut rnd = rand::thread_rng();
        questions.shuffle(&mut rnd);
        Ok(questions)
    }
}
