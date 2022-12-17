#[derive(Clone, Copy, PartialEq, Eq)]
pub enum EnemyType {
    Bomber,
    Fighter,
    Warship(WarshipDirection),
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum WarshipDirection {
    Left,
    Right,
}
