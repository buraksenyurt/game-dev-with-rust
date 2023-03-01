#[cfg(test)]
mod test {
    use crate::cell::Cell;
    use crate::grid::Grid;

    #[test]
    fn init_cell_with_neighbors_test() {
        let mut first_cell = Cell::init(1, 1, 1);
        let mut east_cell = Cell::init(1, 2, 0);
        let mut south_cell = Cell::init(2, 1, 2);

        first_cell.east = Some(Box::new(east_cell.clone()));
        first_cell.connect(&mut east_cell, true);
        first_cell.south = Some(Box::new(south_cell.clone()));
        first_cell.connect(&mut south_cell, false);

        let some_count = first_cell
            .get_neighbors()
            .iter()
            .filter(|c| c.is_some())
            .count();
        assert_eq!(some_count, 2);

        let none_count = first_cell
            .get_neighbors()
            .iter()
            .filter(|c| c.is_none())
            .count();
        assert_eq!(none_count, 2);
    }

    #[test]
    fn is_cells_linked_test() {
        let mut first_cell = Cell::init(1, 1, 0);
        let mut east_cell = Cell::init(1, 2, 1);
        let mut south_cell = Cell::init(2, 1, 2);

        first_cell.connect(&mut east_cell, false);
        first_cell.connect(&mut south_cell, false);

        assert_eq!(first_cell.connected(&east_cell), true);
        assert_eq!(first_cell.connected(&south_cell), true);
    }

    #[test]
    fn grid_size_test() {
        let mut grid = Grid::new(4, 4);
        assert_eq!(grid.get_size(), 16);

        let mut grid = Grid::new(4, 6);
        assert_eq!(grid.get_size(), 24);
    }

    #[test]
    fn create_and_arange_cells_test() {
        let mut grid = Grid::new(4, 4);
        grid.prepare();
        grid.arrange();
        assert_eq!(grid.cells[0].mark, 0);
        assert_eq!(grid.cells[4].mark, 4);
        assert_eq!(grid.cells[6].mark, 6);
        assert_eq!(grid.cells[16].mark, 16);

        // 0X0 hücresinin kuzey hücresi olmaz
        assert!(grid.cells[0].north.is_none());
        // güney komşusu olur
        assert!(grid.cells[0].south.is_some());
        // batı komşusu olur
        assert!(grid.cells[0].west.is_none());
        // doğu komşusu olmaz
        assert!(grid.cells[0].east.is_some());

        // 6ncı indis'teki hücrenin kuzey, güney, doğu, batı komşuları olur
        assert!(grid.cells[6].north.is_some());
        assert!(grid.cells[6].south.is_some());
        assert!(grid.cells[6].west.is_some());
        assert!(grid.cells[6].east.is_some());
    }
}
