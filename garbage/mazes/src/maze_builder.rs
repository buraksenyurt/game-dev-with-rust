use crate::cell::Cell;
use crate::grid::Grid;
use rand::Rng;

pub fn with_btree(source: &Grid) -> Grid {
    let mut grid = Grid::new(source.get_dimension());
    let mut rng = rand::thread_rng();
    for row in source.iter_rows() {
        for cell in row.iter() {
            let mut new_cell = Cell::init(cell.row, cell.column, cell.mark);
            let mut neighbors = Vec::new();
            if let Some(north) = &cell.north {
                neighbors.push(north);
            }
            if let Some(east) = &cell.east {
                neighbors.push(east);
            }
            let index = rng.gen_range(0..neighbors.len());
            let mut connect_cell = Cell::init(
                neighbors[index].row,
                neighbors[index].column,
                neighbors[index].mark,
            );
            new_cell.connect(&mut connect_cell, false);
            grid.cells.push(new_cell);
        }
    }
    grid
}
