pub const RACKET_H: f32 = 75.;
pub const RACKET_W: f32 = 15.;
pub const RACKET_W_HALF: f32 = RACKET_W * 0.5;
pub const RACKET_H_HALF: f32 = RACKET_H * 0.5;
pub const BALL_SIZE: f32 = 32.;
pub const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
pub const PLAYER_SPEED: f32 = 500.;
pub const BALL_SPEED: f32 = 400.;
pub const CENTER_LINE_WIDTH: f32 = 2.;
pub const PADDING: f32 = 10.;
pub const P1_BONUS_IMAGE: &str = "/cube_potions_red.png";
pub const P2_BONUS_IMAGE: &str = "/cube_potions_blue.png";

pub enum Direction {
    Up,
    Down,
}

impl From<Direction> for f32 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => -1.,
            Direction::Down => 1.,
        }
    }
}
