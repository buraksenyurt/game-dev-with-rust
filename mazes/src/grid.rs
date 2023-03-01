use crate::cell::Cell;
use crate::grid_row::GridRow;

pub struct Grid {
    row_count: i32,
    column_count: i32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: i32, columns: i32) -> Self {
        Self {
            row_count: rows,
            column_count: columns,
            cells: vec![],
        }
    }
    pub fn get_size(&self) -> i32 {
        self.row_count * self.column_count
    }
    pub fn get_column_count(&self) -> i32 {
        self.column_count
    }
    pub fn get_row_count(&self) -> i32 {
        self.row_count
    }
    pub fn prepare(&mut self) {
        let mut index = 0;
        for x in 0..self.column_count {
            for y in 0..self.row_count {
                self.cells.push(Cell::init(x, y, index));
                index += 1;
            }
        }
        //println!("Last index number {}",index);
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

    pub fn iter_rows(&self) -> GridRow {
        GridRow {
            grid: &self,
            line_number: 0,
        }
    }
}
