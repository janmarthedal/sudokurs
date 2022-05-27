type CellValue = usize;
type MaskValue = usize;
const FULL_MASK: MaskValue = 511;

struct Board {
    cells: [CellValue; 81],
    column_masks: [MaskValue; 9],
    row_masks: [MaskValue; 9],
    group_masks: [MaskValue; 9],
}

fn index_to_row_column_group(index: usize) -> (usize, usize, usize) {
    let row = index / 9;
    let col = index % 9;
    let group = (row / 3) * 3 + (col / 3);
    (row, col, group)
}

fn value_to_mask(value: CellValue) -> MaskValue {
    1 << (value - 1)
}

fn mask_to_count(mask: MaskValue) -> u32 {
    mask.count_ones()
}

impl Board {
    fn new() -> Self {
        Self {
            cells: [0; 81],
            column_masks: [FULL_MASK; 9],
            row_masks: [FULL_MASK; 9],
            group_masks: [FULL_MASK; 9],
        }
    }
    fn set_at_index(&mut self, index: usize, value: CellValue) {
        self.cells[index] = value;
        let (r, c, g) = index_to_row_column_group(index);
        let mask = value_to_mask(value);
        debug_assert!(self.row_masks[r] & mask != 0);
        debug_assert!(self.column_masks[c] & mask != 0);
        debug_assert!(self.group_masks[g] & mask != 0);
        self.row_masks[r] &= !mask;
        self.column_masks[c] &= !mask;
        self.group_masks[g] &= !mask;
    }
    fn clear_at_index(&mut self, index: usize) {
        let value = self.cells[index];
        self.cells[index] = 0;
        let (r, c, g) = index_to_row_column_group(index);
        let mask = value_to_mask(value);
        self.row_masks[r] |= mask;
        self.column_masks[c] |= mask;
        self.group_masks[g] |= mask;
    }
    fn search_solution(&mut self) {
        // index, count, mask
        let mut best: Option<(usize, u32, MaskValue)> = None;
        for (index, &v) in self.cells.iter().enumerate() {
            if v != 0 {
                continue;
            }
            let (r, c, g) = index_to_row_column_group(index);
            let mask = self.row_masks[r] & self.column_masks[c] & self.group_masks[g];
            if mask == 0 {
                // no solution
                return;
            }
            let count = mask_to_count(mask);
            match best {
                Some((_, c, _)) => {
                    if count < c {
                        best = Some((index, count, mask));
                    }
                }
                None => best = Some((index, count, mask))
            }
        }
        if let Some((index, _, mask)) = best {
            // println!("Try: {} {} {}", i, c, m);
            for value in 1..=9 {
                if value_to_mask(value) & mask != 0 {
                    println!("Set index {} to {}", index, value);
                    self.set_at_index(index, value);
                    self.search_solution();
                    println!("Clear index {}", index);
                    self.clear_at_index(index);
                }
            }
        } else {
            println!("Solution:");
            self.show();
        }
    }
    fn show(&self) {
        for (i, &v) in self.cells.iter().enumerate() {
            print!(
                "{}",
                if v == 0 {
                    '.'
                } else {
                    std::char::from_digit(v as u32, 10).unwrap()
                }
            );
            if (i + 1) % 27 == 0 && i < 80 {
                print!("\n-----------\n")
            } else if (i + 1) % 9 == 0 {
                print!("\n");
            } else if (i + 1) % 3 == 0 {
                print!("|");
            }
        }
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
                _ => panic!("Unexpected char")
            }
        }
        board
    }
}

fn main() {
    /*let mut board = Board::parse("\
        5..26..37\
        .6.4.....\
        ..43..61.\
        95867....\
        1..895746\
        ....2..9.\
        ..1.3...2\
        7.5.41.63\
        6...8.17.\
    ");*/
    let mut board = Board::parse("\
        ....68.3.\
        19.......\
        8.31..2..\
        4...51.6.\
        7...2...4\
        ....7.8..\
        .1...5..7\
        ..4......\
        .5..3.1..\
    ");
    board.show();
    board.show_masks();

    board.search_solution();
}
