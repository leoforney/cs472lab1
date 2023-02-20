use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Cell {
    pub(crate) num: u8,
    pub(crate) x: u8,
    pub(crate) y: u8
}

pub struct Table {
    pub(crate) cells: Vec<Cell>
}

impl Table {
    pub(crate) fn update_or_set_cell(&mut self, cell: Cell) {
        if let Some(existing_cell) = self.cells.iter_mut().find(|c| c.x == cell.x && c.y == cell.y) {
            existing_cell.num = cell.num;
        } else {
            self.cells.push(cell);
        }
    }

    pub(crate) fn parse_file(&mut self) -> Table {
        let mut table = Table { cells: vec![] };
        let file = File::open("file.txt").unwrap();
        let reader = BufReader::new(file);

        for (y, el) in reader.lines().enumerate() {
            let text = el.unwrap();
            for (x, el2) in text.into_iter().enumerate() {
                println!("{} {} {}", x, y, el2);
            }

        }

        table
    }
}