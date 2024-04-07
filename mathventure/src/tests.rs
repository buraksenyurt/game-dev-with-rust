#[cfg(test)]
mod tests {
    use crate::entity::*;
    use crate::utility::{get_index, get_position};

    #[test]
    fn load_map_from_string_content_test() {
        let mut level_1_map = Map::new(1, 9, 5);
        let map_content = "twwwwwwww\
        \ntwtwtttte\
        \nwtttttwtt\
        \ntttwtqwww\
        \nwwwwwwwww";
        level_1_map.load(map_content);
        let entity = &level_1_map.entities[1];
        assert!(entity.as_any().is::<Wall>());

        let entity = &level_1_map.entities[0];
        assert!(entity.as_any().is::<Tile>());

        let entity = &level_1_map.entities[9];
        assert!(entity.as_any().is::<Tile>());

        let entity = &level_1_map.entities[17];
        assert!(entity.as_any().is::<ExitDoor>());

        let entity = &level_1_map.entities[24];
        assert!(entity.as_any().is::<Wall>());

        let entity = &level_1_map.entities[32];
        assert!(entity.as_any().is::<QuestionTower>());

        let entity = &level_1_map.entities[33];
        assert!(entity.as_any().is::<Wall>());
    }

    #[test]
    fn get_cell_index_from_position_test() {
        let x = 32;
        let y = 64;
        let index = get_index(x, y, 9, 32, 32);
        assert_eq!(index, 19);
    }

    #[test]
    fn get_cell_position_from_index_test() {
        let index = 19;
        let position = get_position(index, 9, 32, 32);
        assert_eq!(position.0, 32);
        assert_eq!(position.1, 64);
    }
}
