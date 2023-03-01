use std::collections::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq, PartialEq)]
pub struct Cell {
    pub row: i32,
    pub column: i32,
    pub north: Option<Box<Cell>>,
    pub south: Option<Box<Cell>>,
    pub east: Option<Box<Cell>>,
    pub west: Option<Box<Cell>>,
    links: HashMap<Box<Cell>, bool>,
    pub mark: i32,
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
