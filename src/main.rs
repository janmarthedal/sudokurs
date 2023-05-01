mod bitset;

use std::{fmt::{self, Write}, io::Read};

type CellValue = usize;
type BitSet = bitset::BitSet<usize>;

struct Board {
    cells: [CellValue; 81],
    column_set: [BitSet; 9],
    row_set: [BitSet; 9],
    block_set: [BitSet; 9],
    boards_seen: usize,
}

fn index_to_row_column_block(index: usize) -> (usize, usize, usize) {
    let row = index / 9;
    let col = index % 9;
    let block = (row / 3) * 3 + (col / 3);
    (row, col, block)
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
            block_set: [all; 9],
            boards_seen: 0,
        }
    }

    fn legal_at_index(&self, index: usize, value: CellValue) -> bool {
        let (r, c, g) = index_to_row_column_block(index);
        self.row_set[r].contains(value) && self.column_set[c].contains(value) && self.block_set[g].contains(value)
    }

    fn set_at_index(&mut self, index: usize, value: CellValue) {
        self.cells[index] = value;
        let (r, c, g) = index_to_row_column_block(index);
        self.row_set[r].remove(value);
        self.column_set[c].remove(value);
        self.block_set[g].remove(value);
    }

    fn clear_at_index(&mut self, index: usize) {
        let value = self.cells[index];
        self.cells[index] = 0;
        let (r, c, g) = index_to_row_column_block(index);
        self.row_set[r].insert(value);
        self.column_set[c].insert(value);
        self.block_set[g].insert(value);
    }

    fn search_solution(&mut self) {
        self.boards_seen += 1;
        // index, count, mask
        let mut best: Option<(usize, usize, BitSet)> = None;
        for (index, _) in self.cells.iter().enumerate().filter(|(_, &v)| v == 0) {
            let (r, c, g) = index_to_row_column_block(index);
            let moves = self.row_set[r].intersection(self.column_set[c]).intersection(self.block_set[g]);
            let count = moves.count();
            if count == 0 {
                // no solutions
                return;
            }
            match best {
                Some((_, c, _)) if count >= c => {},
                _ => best = Some((index, count, moves)),
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
        println!("Block masks : {:?}", self.block_set);
    }

    fn get_boards_seen(&self) -> usize {
        self.boards_seen
    }

    fn parse<S: Iterator<Item=char>>(s: S) -> Result<Board, String> {
        let mut board = Board::new();
        let mut board_idx = 0;
        for (input_idx, chr) in s.into_iter().enumerate() {
            match chr {
                '1'..='9' => {
                    let value = chr.to_digit(10).unwrap() as usize;
                    if !board.legal_at_index(board_idx, value) {
                        let (r, c, _) = index_to_row_column_block(board_idx);
                        return Err(format!("Illegal value {value} at row {}, column {}", r + 1, c + 1));
                    }
                    board.set_at_index(board_idx, value);
                    board_idx += 1;
                },
                '.' | ' ' => {
                    // empty cell
                    board_idx += 1;
                },
                '\n' | '|' | '-' => {
                    // ignore
                }
                _ => return Err(format!("Illegal char '{chr}' at index {input_idx}")),
            }
        }
        if board_idx != 81 {
            return Err(format!("Expected 81 cells, got {board_idx}"));
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
    let mut input = Vec::new();
    let stream = std::io::stdin();
    let mut handle = stream.lock();
    handle.read_to_end(&mut input).expect("Failed to read stdin");
    let input = String::from_utf8(input).expect("Failed to parse input as utf8");

    match Board::parse(input.chars()) {
        Ok(mut board) => {
            println!("{}", board);
            board.show_masks();
            board.search_solution();
            println!("\nBoards seen: {}", board.get_boards_seen());
        },
        Err(err) => println!("Error: {}", err),
    }
}
