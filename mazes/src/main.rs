/*
   Çalışmakta olduğum Mazes for Programmers kitabını uygulamaya çalışıyorum.
   İlk örnekte binary tree algoritmasına göre labirent oluşturulması ele alınıyor.
   Hatta başlangıç noktasından bitişe ulaşmak için Dijkstra algoritması kullanılıyor.
   Sorun şu ki kitap örnek kodları Ruby ile yazmış. Rust'a evirmek pek kolay değil.
*/
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

fn main() {}

#[derive(Clone, Eq, PartialEq)]
struct Cell {
    row: i32,
    column: i32,
    north: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    links: HashMap<Box<Cell>, bool>,
    mark: i32,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.column.hash(state);
        self.row.hash(state);
    }
}

impl Cell {
    pub fn init(row: i32, column: i32, mark: i32) -> Self {
        Self {
            row,
            column,
            north: None,
            south: None,
            west: None,
            east: None,
            links: HashMap::new(),
            mark,
        }
    }

    pub fn links(&self) -> Vec<Box<Cell>> {
        self.links.keys().cloned().collect()
    }

    pub fn connected(&self, cell: &Cell) -> bool {
        self.links.contains_key(&Box::new(cell.clone()))
    }

    pub fn connect(&mut self, cell: &mut Cell, bidirectional: bool) -> &mut Self {
        self.links.insert(Box::new(cell.clone()), true);
        if bidirectional {
            cell.connect(self, false);
        }
        self
    }

    pub fn disconnect(&mut self, cell: &mut Cell, bidirectional: bool) -> &mut Self {
        self.links.remove(&Box::new(cell.clone()));
        if bidirectional {
            cell.disconnect(self, false);
        }
        self
    }

    pub fn get_neighbors(&self) -> Vec<Option<Box<Cell>>> {
        let mut results = vec![
            self.north.clone(),
            self.south.clone(),
            self.east.clone(),
            self.west.clone(),
        ];
        results
    }
}

struct Grid {
    row_count: i32,
    column_count: i32,
    cells: Vec<Cell>,
}
impl Grid {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            row_count: rows,
            column_count: columns,
            cells: Vec::new(),
        }
    }
    pub fn prepare(&mut self) {
        let mut index = 0;
        for x in 0..=self.column_count {
            for y in 0..=self.row_count {
                self.cells.push(Cell::init(x, y, index));
                index += 1;
            }
        }
    }
    fn get_index(&self, row: i32, column: i32) -> i32 {
        (row * self.column_count) + column
    }

    /*
       x,y koordinatlarını vector'deki indis değerine dönüştürmek için bir yol olabilir.

       -----------------
       | 0 | 1 | 2 | 3 |
       -----------------
       | 4 | 5 | 6 | 7 |
       -----------------
       | 8 | 9 |10 |11 |
       -----------------
       |12 |13 |14| 15 |
       -----------------> X Axis
       |
       Y aksis

       let index = (y * WIDTH) + x;
       let x = index % WIDTH;
       let y = index / WIDTH;
    */

    pub fn arrange(&mut self) {
        for c in self.cells.clone().iter_mut() {
            let (row, column) = (c.row, c.column);
            println!("(row X column) = ( {} X {} )", row, column);
            let index = self.get_index(row, column) as usize;
            let north_index = self.get_index(row - 1, column);
            let south_index = self.get_index(row + 1, column);
            let west_index = self.get_index(row, column - 1);
            let east_index = self.get_index(row, column + 1);

            if north_index > 0 {
                self.cells[index].north = Some(Box::new(self.cells[north_index as usize].clone()));
            }
            if south_index > 0 {
                self.cells[index].south = Some(Box::new(self.cells[south_index as usize].clone()));
            }
            if east_index > 0 {
                self.cells[index].east = Some(Box::new(self.cells[east_index as usize].clone()));
            }
            if west_index > 0 {
                self.cells[index].west = Some(Box::new(self.cells[west_index as usize].clone()));
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{Cell, Grid};

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
    fn create_and_arange_cells_test() {
        let mut grid = Grid::new(4, 4);
        grid.prepare();
        grid.arrange();
        assert_eq!(grid.cells[0].mark, 0);
        assert_eq!(grid.cells[4].mark, 4);
        assert_eq!(grid.cells[6].mark, 6);
        assert_eq!(grid.cells[16].mark, 16);

        assert!(grid.cells[0].north.is_none());
        assert!(grid.cells[0].south.is_some());
        assert!(grid.cells[0].west.is_none());
        assert!(grid.cells[0].east.is_some());

        assert!(grid.cells[6].north.is_some());
        assert!(grid.cells[6].south.is_some());
        assert!(grid.cells[6].west.is_some());
        assert!(grid.cells[6].east.is_some());
    }
}
