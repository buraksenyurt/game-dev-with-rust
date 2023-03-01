#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Dot,
    Empty,
    Powerup,
}
#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    Wall,
    NotWall(TileType),
    Room,
}
