use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub play_commands: Vec<CommandPair>,
    pub menu_commands: Vec<CommandPair>,
}

#[derive(Serialize, Deserialize)]
pub struct CommandPair {
    pub key: String,
    pub command: String,
}
