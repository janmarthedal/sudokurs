mod bitset;

use std::fmt::{self, Write};
use bitset::BitSet;

type CellValue = usize;

struct Board {
    cells: [CellValue; 81],
    column_masks: [BitSet; 9],
    row_masks: [BitSet; 9],
    group_masks: [BitSet; 9],
}

fn index_to_row_column_group(index: usize) -> (usize, usize, usize) {
    let row = index / 9;
    let col = index % 9;
    let group = (row / 3) * 3 + (col / 3);
    (row, col, group)
}

impl Board {
    fn new() -> Self {
        let mut mask = BitSet::new();
        for i in 1..=9 {
            mask.insert(i);
        }
        Self {
            cells: [0; 81],
            column_masks: [mask; 9],
            row_masks: [mask; 9],
            group_masks: [mask; 9],
        }
    }
    fn set_at_index(&mut self, index: usize, value: CellValue) {
        self.cells[index] = value;
        let (r, c, g) = index_to_row_column_group(index);
        debug_assert!(self.row_masks[r].contains(value));
        debug_assert!(self.column_masks[c].contains(value));
        debug_assert!(self.group_masks[g].contains(value));
        self.row_masks[r].remove(value);
        self.column_masks[c].remove(value);
        self.group_masks[g].remove(value);
    }
    fn clear_at_index(&mut self, index: usize) {
        let value = self.cells[index];
        self.cells[index] = 0;
        let (r, c, g) = index_to_row_column_group(index);
        self.row_masks[r].insert(value);
        self.column_masks[c].insert(value);
        self.group_masks[g].insert(value);
    }
    fn search_solution(&mut self) -> usize {
        // index, count, mask
        let mut best: Option<(usize, usize, BitSet)> = None;
        for (index, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                continue;
            }
            let (r, c, g) = index_to_row_column_group(index);
            let legal = self.row_masks[r].intersection(self.column_masks[c]).intersection(self.group_masks[g]);
            if legal.is_empty() {
                // no solutions
                return 1;
            }
            let count = legal.count();
            match best {
                Some((_, c, _)) => {
                    if count < c {
                        best = Some((index, count, legal));
                    }
                }
                None => best = Some((index, count, legal)),
            }
        }
        if let Some((index, _, moves)) = best {
            let mut calls = 1;
            for value in 1..=9 {
                if moves.contains(value) {
                    self.set_at_index(index, value);
                    calls += self.search_solution();
                    self.clear_at_index(index);
                }
            }
            return calls;
        }
        println!("\nSolution:\n{self}");
        1
    }
    fn show_masks(&self) {
        println!("Row masks   : {:?}", self.row_masks);
        println!("Column masks: {:?}", self.column_masks);
        println!("Group masks : {:?}", self.group_masks);
    }
    fn parse(s: &str) -> Board {
        assert_eq!(s.len(), 81);
        let mut board = Board::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '1'..='9' => board.set_at_index(i, c.to_digit(10).unwrap() as usize),
                '.' => {}
                _ => panic!("Unexpected char"),
            }
        }
        board
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
    // let mut board = Board::parse("\
    //     8........\
    //     ..36.....\
    //     .7..9.2..\
    //     .5...7...\
    //     ....457..\
    //     ...1...3.\
    //     ..1....68\
    //     ..85...1.\
    //     .9....4..\
    // ");
    let mut board = Board::parse(
        "\
        29.....87\
        ....8....\
        ..527..41\
        ...9..1.6\
        ..1...9..\
        9.4..6...\
        76..384..\
        ....9....\
        31.....98\
    ",
    );
    println!("{}", board);
    board.show_masks();

    let calls = board.search_solution();
    println!("\nDifficulty: {}", calls);
}
