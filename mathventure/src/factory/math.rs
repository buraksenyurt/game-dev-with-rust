pub struct Math;

#[derive(Clone)]
pub struct Dimension {
    pub width: u32,
    pub height: u32,
}
impl Dimension {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

pub struct Vector {
    pub x: f32,
    pub y: f32,
}
impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct Location {
    pub x: i32,
    pub y: i32,
}
impl Location {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl Math {
    pub fn get_index(x: u32, y: u32, dimension: Dimension, col_count: u32) -> u32 {
        (y / dimension.height) * col_count + (x / dimension.width)
    }

    pub fn get_position(index: u32, dimension: Dimension, col_count: u32) -> (u32, u32) {
        (
            (index % col_count) * dimension.width,
            (index / col_count) * dimension.height,
        )
    }

    pub fn get_unit_vector(a: Vector, b: Vector) -> Vector {
        let vec_ab = (b.x - a.x, b.y - a.y);
        let magnitude_ab = ((vec_ab.0).powf(2.0) + (vec_ab.1).powf(2.0)).sqrt();
        Vector::new(vec_ab.0 / magnitude_ab, vec_ab.1 / magnitude_ab)
    }
}
