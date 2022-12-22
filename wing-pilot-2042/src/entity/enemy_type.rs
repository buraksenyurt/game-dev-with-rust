#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Bomber,
    Fighter,
    Warship(Option<WarshipDirection>),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WarshipDirection {
    Left,
    Right,
}
