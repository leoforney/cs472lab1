use crate::tableparser::{Cell, Table};

mod tableparser;

fn main() {
    println!("Hello, world!");
    let mut table = Table { cells: vec![] };
    let cell1 = Cell { num: 1, x: 0, y: 0 };
    let cell2 = Cell { num: 2, x: 1, y: 0 };
    let cell3 = Cell { num: 3, x: 0, y: 1 };
    table.update_or_set_cell(cell1);
    table.update_or_set_cell(cell2);
    table.update_or_set_cell(cell3);
    println!("{:?}", table.cells);

}
