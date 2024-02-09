use std::str::FromStr;

pub enum MainMenu {
    NewGame,
    LoadGame,
    TransferMarket,
    ExitGame,
}

impl FromStr for MainMenu {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "N" => Ok(Self::NewGame),
            "L" => Ok(Self::LoadGame),
            "T" => Ok(Self::TransferMarket),
            "X" => Ok(Self::ExitGame),
            _ => Err("Invalid command".to_string()),
        }
    }
}
