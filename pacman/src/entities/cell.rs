#[derive(PartialEq, Copy, Clone, Debug)]
pub enum TileType {
    Dot,
    Empty,
    Powerup,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Tile {
    Wall,
    NotWall(TileType),
    Room,
}
