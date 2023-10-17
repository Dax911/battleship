extern crate prettytable;

use prettytable::format;
use prettytable::{Cell, Row, Table};

#[derive(Debug)]
pub enum Battleship {
    Carrier,
    Battleship,
    Cruiser,
    Submarine,
    Destroyer,
}

impl Battleship {
    fn size(&self) -> u8 {
        match self {
            Battleship::Carrier => 5,
            Battleship::Battleship => 4,
            Battleship::Cruiser => 3,
            Battleship::Submarine => 3,
            Battleship::Destroyer => 2,
        }
    }
}
fn main() {
    let ships = [
        Battleship::Carrier,
        Battleship::Battleship,
        Battleship::Cruiser,
        Battleship::Submarine,
        Battleship::Destroyer,
    ];

    for ship in &ships {
        println!("{:?} has size {}", ship, ship.size());
    }

    let mut board: [[i32; 10]; 10] = [[0; 10]; 10];
    loop {
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim the input and convert to uppercase for consistency
        let input = input.trim().to_uppercase();

        // Parse the input to get row and column
        if input.len() >= 2 {
            let (row_char, col_str) = input.split_at(1);
            if let Ok(col) = col_str.parse::<usize>() {
                let row = row_char.chars().next().unwrap() as usize - 'A' as usize;
                if row < 10 && col < 10 {
                    board[row][col] = 1; // Mark the cell as "fired upon"
                }
            }
        }

        let mut table = Table::new();

        // Add header with numbers
        let mut header = Row::empty();
        header.add_cell(Cell::new(" "));
        for i in 0..10 {
            header.add_cell(Cell::new(&i.to_string()).style_spec("c"));
        }
        table.add_row(header);

        // Add rows with data and letters on the side
        for (i, row) in board.iter().enumerate() {
            let mut table_row = Row::empty();
            let letter = (('A' as u8 + i as u8) as char).to_string();
            let cell_style = if i % 2 == 0 { "c" } else { "cFg:yellow" }; // Alternate colors
            table_row.add_cell(Cell::new(&letter).style_spec(cell_style));
            for &col in row {
                let content: String = if col == 0 {
                    "".to_string()
                } else {
                    col.to_string()
                };
                table_row.add_cell(Cell::new(&content).style_spec(cell_style));
            }
            table.add_row(table_row);
        }

        table.printstd();
    }
}
