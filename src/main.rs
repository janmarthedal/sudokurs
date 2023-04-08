mod bitset;

use std::fmt::{self, Write};
use bitset::BitSet;

type CellValue = usize;

struct Board {
    cells: [CellValue; 81],
    column_set: [BitSet; 9],
    row_set: [BitSet; 9],
    group_set: [BitSet; 9],
    boards_seen: usize,
}

fn index_to_row_column_group(index: usize) -> (usize, usize, usize) {
    let row = index / 9;
    let col = index % 9;
    let group = (row / 3) * 3 + (col / 3);
    (row, col, group)
}

impl Board {
    fn new() -> Self {
        let mut all = BitSet::new();
        for i in 1..=9 {
            all.insert(i);
        }
        Self {
            cells: [0; 81],
            column_set: [all; 9],
            row_set: [all; 9],
            group_set: [all; 9],
            boards_seen: 0,
        }
    }
    fn legal_at_index(&self, index: usize, value: CellValue) -> bool {
        let (r, c, g) = index_to_row_column_group(index);
        self.row_set[r].contains(value) && self.column_set[c].contains(value) && self.group_set[g].contains(value)
    }
    fn set_at_index(&mut self, index: usize, value: CellValue) {
        self.cells[index] = value;
        let (r, c, g) = index_to_row_column_group(index);
        self.row_set[r].remove(value);
        self.column_set[c].remove(value);
        self.group_set[g].remove(value);
    }
    fn clear_at_index(&mut self, index: usize) {
        let value = self.cells[index];
        self.cells[index] = 0;
        let (r, c, g) = index_to_row_column_group(index);
        self.row_set[r].insert(value);
        self.column_set[c].insert(value);
        self.group_set[g].insert(value);
    }
    fn search_solution(&mut self) {
        self.boards_seen += 1;
        // index, count, mask
        let mut best: Option<(usize, u32, BitSet)> = None;
        for (index, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                continue;
            }
            let (r, c, g) = index_to_row_column_group(index);
            let moves = self.row_set[r].intersection(self.column_set[c]).intersection(self.group_set[g]);
            if moves.is_empty() {
                // no solutions
                return;
            }
            let count = moves.count();
            match best {
                Some((_, c, _)) if count < c => best = Some((index, count, moves)),
                None => best = Some((index, count, moves)),
                _ => {}
            }
        }
        if let Some((index, _, moves)) = best {
            for value in moves.iter() {
                self.set_at_index(index, value);
                self.search_solution();
                self.clear_at_index(index);
            }
        } else {
            println!("\nSolution:\n{self}");
        }
    }
    fn show_masks(&self) {
        println!("Row masks   : {:?}", self.row_set);
        println!("Column masks: {:?}", self.column_set);
        println!("Group masks : {:?}", self.group_set);
    }
    fn get_boards_seen(&self) -> usize {
        self.boards_seen
    }
    fn parse(s: &str) -> Result<Board, String> {
        if s.len() != 81 {
            return Err(format!("Expected 81 chars, got {}", s.len()));
        }
        let mut board = Board::new();
        for (i, chr) in s.chars().enumerate() {
            match chr {
                '1'..='9' => {
                    let value = chr.to_digit(10).unwrap() as usize;
                    if !board.legal_at_index(i, value) {
                        let (r, c, _) = index_to_row_column_group(i);
                        return Err(format!("Illegal value {value} at row {}, column {}", r + 1, c + 1));
                    }
                    board.set_at_index(i, value);
                },
                '.' | ' ' => {}
                _ => return Err(format!("Illegal char '{chr}' at index {i}")),
            }
        }
        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, &v) in self.cells.iter().enumerate() {
            f.write_char(if v == 0 {
                '.'
            } else {
                std::char::from_digit(v as u32, 10).unwrap()
            })?;
            if i == 80 {
                // skip
            } else if (i + 1) % 27 == 0 {
                f.write_str("\n-----------\n")?;
            } else if (i + 1) % 9 == 0 {
                f.write_char('\n')?;
            } else if (i + 1) % 3 == 0 {
                f.write_char('|')?;
            }
        }
        Ok(())
    }
}

fn main() {
    // Arto Inkala (https://abcnews.go.com/blogs/headlines/2012/06/can-you-solve-the-hardest-ever-sudoku)
    // let board_setup = "\
    //     8........\
    //     ..36.....\
    //     .7..9.2..\
    //     .5...7...\
    //     ....457..\
    //     ...1...3.\
    //     ..1....68\
    //     ..85...1.\
    //     .9....4..\
    // ";
    let board_setup = "\
        29.....87\
        ....8....\
        ..527..41\
        ...9..1.6\
        ..1...9..\
        9.4..6...\
        76..384..\
        ....9....\
        31.....98\
    ";
    let mut board = Board::parse(board_setup).expect("Failed to parse");
    println!("{}", board);
    board.show_masks();

    board.search_solution();
    println!("\nBoards seen: {}", board.get_boards_seen());
}
