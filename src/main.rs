use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut sudoku_area: HashMap<usize, HashSet<char>> = HashMap::new();
        for (counter, row) in board.iter().enumerate() {
            println!("{:?}", counter);
            println!("{:?}", row);
            if counter % 3 == 0 {
                println!("{:?}", &row[0..3]);
                sudoku_area.insert(counter, row[0..3].iter().copied().collect());

                sudoku_area.insert(counter + 1, row[3..6].iter().copied().collect());

                sudoku_area.insert(counter + 2, row[6..9].iter().copied().collect());
            }
            println!("{:?}", sudoku_area);
            for (column_counter, column) in row.iter().enumerate() {
                println!("{:?}", column_counter);
                match column_counter {
                    0..3 => println!(),
                    3..6 => println!(),
                    6..9 => println!(),
                }
            }
        }

        true
    }
}

fn main() {}
