#[cfg(test)]
pub mod test {
    use crate::common::contants::{TILE_MAP_HEIGHT, TILE_MAP_WIDTH};
    use crate::common::position::Position;
    use crate::entities::cell::Tile::{NotWall, Wall};
    use crate::entities::cell::TileType::Dot;
    use crate::entities::cell::{Tile, TileType};
    use crate::entities::map::Map;
    use crate::entities::pacman::Pacman;
    use graphics::math::dot;

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

    #[test]
    fn position_test() {
        let map = Map::default();

        let pos = Position::new(-1, -1);
        let get_result = map.get(&pos);
        assert_eq!(get_result, None);

        let pos = Position::new(10, TILE_MAP_HEIGHT as i32 + 1);
        let get_result = map.get(&pos);
        assert_eq!(get_result, None);

        let pos = Position::new(1, 1);
        let get_result = map.get(&pos);
        assert_eq!(get_result, Some(NotWall(Dot)));

        let pos = Position::new(2, 2);
        let get_result = map.get(&pos);
        assert_eq!(get_result, Some(Wall));
    }

    #[test]
    fn how_many_tiles() {
        let mut wall_count = 0;
        let mut dot_count = 0;
        let mut room_count = 0;
        let mut powerup_count = 0;
        let mut empty_count = 0;
        let map = Map::default();

        for line in map.read_lines() {
            for tile in line.iter() {
                match tile {
                    Wall => wall_count += 1,
                    NotWall(Dot) => dot_count += 1,
                    NotWall(TileType::Powerup) => powerup_count += 1,
                    NotWall(TileType::Empty) => empty_count += 1,
                    Tile::Room => room_count += 1,
                }
            }
        }
        assert_eq!(wall_count, 548);
        assert_eq!(dot_count, 242);
        assert_eq!(room_count, 20);
        assert_eq!(powerup_count, 4);
        assert_eq!(empty_count, 54);
    }
}
