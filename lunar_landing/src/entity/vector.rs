use sdl2::rect::Point;

#[derive(Default, PartialEq, Debug)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn to_point(&self) -> Point {
        Point::new(self.x as i32, self.y as i32)
    }
}
