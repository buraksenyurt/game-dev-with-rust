#[derive(Clone, Copy, PartialEq)]
pub enum EnemyType {
    Bomber,
    Fighter,
    Warship(WarshipDirection),
}

#[derive(Clone, Copy, PartialEq)]
pub enum WarshipDirection {
    Left,
    Right,
}
