use std::{
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
};

struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut sudoku_area: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        let mut cols: HashMap<usize, HashSet<char>> = HashMap::new();
        for (row_counter, row) in board.iter().enumerate() {
            let mut new_row: HashSet<char> = HashSet::new();
            for (column_counter, column) in row.iter().enumerate() {
                if *column == '.' {
                    continue;
                }

                let val = sudoku_area
                    .entry((row_counter / 3, column_counter / 3))
                    .or_default()
                    .insert(*column);

                let res = new_row.insert(*column);
                println!("{:?}", sudoku_area);

                let column_checker_res = cols.entry(column_counter).or_default().insert(*column);
                if !val || !res || !column_checker_res {
                    return false;
                }
            }
        }

        true
    }
}
