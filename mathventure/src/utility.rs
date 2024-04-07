pub fn get_index(x: u32, y: u32, column_count: u32, tile_height: u32, tile_width: u32) -> u32 {
    (y / tile_height) * column_count + (x / tile_width)
}

pub fn get_position(
    index: u32,
    column_count: u32,
    tile_height: u32,
    tile_width: u32,
) -> (u32, u32) {
    (
        (index % column_count) * tile_width,
        (index / column_count) * tile_height,
    )
}
