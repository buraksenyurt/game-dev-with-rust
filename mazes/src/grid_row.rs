use crate::cell::Cell;
use crate::grid::Grid;

pub struct GridRow<'a> {
    pub grid: &'a Grid,
    pub line_number: usize,
}

impl<'a> Iterator for GridRow<'a> {
    type Item = &'a [Cell];

    fn next(&mut self) -> Option<&'a [Cell]> {
        let (col_count, row_count) = (
            self.grid.get_dimension() as usize,
            self.grid.get_dimension() as usize,
        );
        let line_start = self.line_number * row_count;
        self.line_number += 1;
        if line_start >= self.grid.get_size() as usize {
            None
        } else {
            Some(&self.grid.cells[line_start..(line_start + col_count)])
        }
    }
}
