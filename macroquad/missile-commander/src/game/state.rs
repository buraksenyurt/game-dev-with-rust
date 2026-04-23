use crate::stage::stage::Stage;

pub enum State {
    Main,
    Playing(Stage),
    Dead,
    Win,
    End,
}
