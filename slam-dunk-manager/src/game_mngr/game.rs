use crate::prelude::model::*;
use bincode;
use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ProgramState {
    MainMenu,
    InitNewGame,
    Exit,
    None,
    ShowTransferMarket,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub program_state: ProgramState,
    pub fixture: Vec<MatchDay>,
    //pub league:League
}

impl Game {
    pub async fn save(&self, path: &str) -> io::Result<()> {
        let encoded = bincode::serialize(&self)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        let mut file = File::create(path).await?;
        file.write_all(&encoded).await?;
        Ok(())
    }

    pub async fn load(path: &str) -> io::Result<Self> {
        let mut file = File::open(path).await?;
        let mut contents = vec![];
        file.read_to_end(&mut contents).await?;
        let game: Game = bincode::deserialize(&contents)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        Ok(game)
    }
}
