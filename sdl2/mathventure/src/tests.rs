#[cfg(test)]
mod tests {
    use crate::entity::*;
    use crate::factory::{Dimension, Math};
    use crate::game::Game;
    use crate::resources::{LevelManager, BLOCK_HEIGHT, BLOCK_WIDTH, STANDARD_COLUMN_COUNT};

    #[test]
    fn load_map_from_string_content_test() {
        let level_manager = LevelManager::init();
        let level = level_manager.get_level(0).unwrap();
        let mut level_1_map = Map::default();
        let map_content = level.map.as_str();
        level_1_map.load(map_content);

        let entity = &level_1_map.tiles[1];
        assert_eq!(entity.get_type(), BlockType::Wall);

        let entity = &level_1_map.tiles[19];
        assert_eq!(entity.get_type(), BlockType::ExitDoor);

        let entity = &level_1_map.tiles[20];
        assert_eq!(entity.get_type(), BlockType::Wall);

        let entity = &level_1_map.tiles[35];
        assert_eq!(entity.get_type(), BlockType::QuestionTower);
    }

    #[test]
    fn get_player_index_test() {
        let mut game = Game::default();
        game.init(0);
        assert_eq!(game.player.idx, 30);
    }

    #[test]
    fn get_cell_index_from_position_test() {
        let x = 32;
        let y = 64;
        let dimension = Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT);
        let index = Math::get_index(x, y, dimension, STANDARD_COLUMN_COUNT);
        assert_eq!(index, 10);
    }

    #[test]
    fn get_cell_position_from_index_test() {
        let index = 12;
        let dimension = Dimension::new(BLOCK_WIDTH, BLOCK_HEIGHT);
        let position = Math::get_position(index, dimension, STANDARD_COLUMN_COUNT);
        assert_eq!(position.0, 128);
        assert_eq!(position.1, 64);
    }

    #[test]
    fn get_level_by_index_test() {
        let level_manager = LevelManager::init();
        let expected = level_manager.get_level(0);
        assert!(expected.is_some());
    }

    // #[test]
    // fn get_level_by_unknown_index_test() {
    //     let level_manager = LevelManager::init();
    //     let expected = level_manager.get_level(9999);
    //     assert!(expected.is_none());
    // }
}
