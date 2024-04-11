use crate::resources::{BLOCK_HEIGHT, BLOCK_WIDTH, STANDARD_COLUMN_COUNT};

pub fn get_index(x: u32, y: u32) -> u32 {
    (y / BLOCK_HEIGHT) * STANDARD_COLUMN_COUNT + (x / BLOCK_WIDTH)
}

pub fn get_position(index: u32) -> (u32, u32) {
    (
        (index % STANDARD_COLUMN_COUNT) * BLOCK_WIDTH,
        (index / STANDARD_COLUMN_COUNT) * BLOCK_HEIGHT,
    )
}

// pub fn get_unit_vector(xa: f32, ya: f32, xb: f32, yb: f32) -> (f32, f32) {
//     let vec_ab = (xb - xa, yb - ya);
//     let magnitude_ab = ((vec_ab.0).powf(2.0) + (vec_ab.1).powf(2.0)).sqrt();
//     let unit_vec_ab = (vec_ab.0 / magnitude_ab, vec_ab.1 / magnitude_ab);
//     unit_vec_ab
// }
