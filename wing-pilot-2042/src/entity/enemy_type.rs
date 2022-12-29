#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EnemyType {
    Bomber,
    Fighter,
    Warship(Option<WarshipDirection>),
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum WarshipDirection {
    Left,
    Right,
}
