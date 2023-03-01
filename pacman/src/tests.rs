#[cfg(test)]
pub mod test {
    use crate::common::contants::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH};
    use crate::entities::map::Map;
    use crate::entities::pacman::Pacman;

    #[test]
    fn create_ground_test() {
        let map = Map::default();
        assert_eq!(map.tiles.iter().count(), TILE_MAP_WIDTH * TILE_MAP_HEIGHT);
        assert_eq!(map.pellets.count, 242);
        assert_eq!(map.pellets.coordinates.iter().count(), 242);
    }

    #[test]
    fn read_map_line_by_line_test() {
        let map = Map::default();
        let mut row_index = 0;
        for line in map.read_lines() {
            for _cell in line.iter() {}
            row_index += 1;
        }
        assert_eq!(row_index, TILE_MAP_HEIGHT);
    }
}
