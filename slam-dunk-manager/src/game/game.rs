pub struct Game {
    pub current_state: GameState,
}

pub enum GameState {
    Initial,
    MainMenu,
    TransferMarket,
    NewGame,
    TeamChoose,
}
