use adventofcode_2021::get_input;

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    risk: u64,
    pos: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .risk
            .cmp(&self.risk)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let s = get_input().unwrap();
    let grid: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    println!("Part 1: {}", dijkstra(&grid).unwrap());

    let mut full_grid: Vec<Vec<u8>> = Vec::with_capacity(grid.len() * 5);

    for i in 0..5 {
        for y in 0..grid.len() {
            let mut v = Vec::with_capacity(grid.len() * 5);
            for j in 0..5 {
                for x in 0..grid.len() {
                    let mut val = grid[y][x] + i + j;
                    while val > 9 {
                        val -= 9;
                    }
                    v.push(val);
                }
            }
            full_grid.push(v);
        }
    }

    println!("Part 2: {}", dijkstra(&full_grid).unwrap());
}

fn dijkstra(grid: &Vec<Vec<u8>>) -> Option<u64> {
    let start = (0, 0);
    let goal = (grid.len() - 1, grid.len() - 1);
    let mut dist: Vec<Vec<u64>> = (0..grid.len())
        .map(|_| (0..grid.len()).map(|_| u64::MAX).collect())
        .collect();

    let mut heap = BinaryHeap::new();
    dist[start.0][start.1] = 0;
    heap.push(State {
        risk: 0,
        pos: start,
    });

    while let Some(State { risk, pos }) = heap.pop() {
        if pos == goal {
            return Some(risk);
        }

        if risk > dist[pos.0][pos.1] {
            continue;
        }

        let mut possibilities = Vec::new();

        if pos.1 > 0 {
            possibilities.push((pos.0, pos.1 - 1));
        }
        if pos.1 < dist[pos.0].len() - 1 {
            possibilities.push((pos.0, pos.1 + 1))
        }

        if pos.0 > 0 {
            possibilities.push((pos.0 - 1, pos.1));
        }

        if pos.0 < dist.len() - 1 {
            possibilities.push((pos.0 + 1, pos.1));
        }

        for pos in possibilities {
            let next = State {
                risk: risk + grid[pos.0][pos.1] as u64,
                pos,
            };

            if next.risk < dist[pos.0][pos.1] {
                heap.push(next);
                dist[pos.0][pos.1] = next.risk;
            }
        }
    }

    None
}

#[allow(unused)]
fn dump_risks(risks: &Vec<Vec<u64>>) {
    for i in 0..risks.len() {
        for j in 0..risks.len() {
            print!(" {:04} ", risks[i][j]);
        }
        println!("\n");
    }
}

#[allow(unused)]
fn dump_grid(grid: &Vec<Vec<u8>>) {
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            print!("{}", grid[i][j]);
        }
        println!();
    }
}
