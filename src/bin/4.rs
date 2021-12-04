use adventofcode_2021::get_input;

use std::collections::HashSet;

fn main() {
    let s = get_input().unwrap();
    let mut lines = s.lines();
    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut rows = Vec::new();
    for line in lines.skip(1) {
        if line.is_empty() {
            boards.push(Board::new(rows));
            rows = Vec::new();
        } else {
            rows.push(
                line.split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect(),
            );
        }
    }
    boards.push(Board::new(rows));

    let mut winners = HashSet::new();
    let num_boards = boards.len();
    for num in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            board.mark(num);
            if !winners.contains(&i) && board.winner() {
                let score: u64 = num as u64
                    * board
                        .unmarked_numbers()
                        .into_iter()
                        .map(|num| num as u64)
                        .sum::<u64>();

                if winners.is_empty() {
                    println!("Part 1: {}", score);
                }

                winners.insert(i);

                if winners.len() == num_boards {
                    println!("Part 2: {}", score);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Board {
    pub rows: Vec<Vec<u8>>,
    pub marks: HashSet<u8>,
}

impl Board {
    fn new(rows: Vec<Vec<u8>>) -> Self {
        Self {
            rows,
            marks: HashSet::new(),
        }
    }

    fn is_marked(&self, num: &u8) -> bool {
        self.marks.contains(num)
    }

    fn winner(&self) -> bool {
        let mut checked_cols: Vec<bool> = Vec::new();
        for (j, row) in self.rows.iter().enumerate() {
            if row.iter().all(|num| self.is_marked(num)) {
                return true;
            }

            if j == 0 {
                for num in row {
                    checked_cols.push(self.is_marked(num));
                }
            } else {
                for (i, num) in row.iter().enumerate() {
                    if checked_cols[i] && !self.is_marked(num) {
                        checked_cols[i] = false;
                    }
                }
            }
        }

        if checked_cols.iter().any(|col| *col) {
            return true;
        }

        false
    }

    fn mark(&mut self, num: u8) {
        self.marks.insert(num);
    }

    fn unmarked_numbers(&self) -> Vec<u8> {
        self.rows
            .iter()
            .flat_map(|row| row.iter().filter(|num| !self.is_marked(num)).copied())
            .collect()
    }
}
