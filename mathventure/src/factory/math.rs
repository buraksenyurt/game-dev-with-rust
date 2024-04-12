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

pub struct Rectangle {
    pub location: Location,
    pub dimension: Dimension,
}

impl Rectangle {
    pub fn new(location: Location, dimension: Dimension) -> Self {
        Self {
            location,
            dimension,
        }
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

    // Axis-Aligned Bounding Box AABB hesaplama tekniğine göre
    pub fn check_collision(rectangle_1: Rectangle, rectangle_2: Rectangle) -> bool {
        let rect1_right = rectangle_1.location.x + rectangle_1.dimension.width as i32;
        let rect1_bottom = rectangle_1.location.y + rectangle_1.dimension.height as i32;
        let rect2_right = rectangle_2.location.x + rectangle_2.dimension.width as i32;
        let rect2_bottom = rectangle_2.location.y + rectangle_2.dimension.height as i32;

        if rectangle_1.location.x < rect2_right
            && rect1_right > rectangle_2.location.x
            && rectangle_1.location.y < rect2_bottom
            && rect1_bottom > rectangle_2.location.y
        {
            true
        } else {
            false
        }
    }
}
