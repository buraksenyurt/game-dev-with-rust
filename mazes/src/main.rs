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
    row: u16,
    column: u16,
    north: Option<Box<Cell>>,
    south: Option<Box<Cell>>,
    east: Option<Box<Cell>>,
    west: Option<Box<Cell>>,
    links: HashMap<Box<Cell>, bool>,
}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.column.hash(state);
        self.row.hash(state);
    }
}

impl Cell {
    pub fn init(row: u16, column: u16) -> Self {
        Self {
            row,
            column,
            north: None,
            south: None,
            west: None,
            east: None,
            links: HashMap::new(),
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
        let mut result = Vec::new();
        result.push(self.north.clone());
        result.push(self.south.clone());
        result.push(self.east.clone());
        result.push(self.west.clone());
        result
    }
}

#[cfg(test)]
mod test {
    use crate::Cell;

    #[test]
    fn init_cell_with_neighbors_test() {
        let mut first_cell = Cell::init(1, 1);
        let mut east_cell = Cell::init(1, 2);
        let mut south_cell = Cell::init(2, 1);

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
        let mut first_cell = Cell::init(1, 1);
        let mut east_cell = Cell::init(1, 2);
        let mut south_cell = Cell::init(2, 1);

        first_cell.connect(&mut east_cell, false);
        first_cell.connect(&mut south_cell, false);

        assert_eq!(first_cell.connected(&east_cell), true);
        assert_eq!(first_cell.connected(&south_cell), true);
    }
}
