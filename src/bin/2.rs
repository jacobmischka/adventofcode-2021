use std::str::FromStr;

use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();
    let sub1 = Submarine::new(&s, false).unwrap();
    println!("Part 1: {}", sub1.y * sub1.z);
    let sub2 = Submarine::new(&s, true).unwrap();
    println!("Part 2: {}", sub2.y * sub2.z);
}

enum Direction {
    Forward,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Direction::Forward),
            "up" => Ok(Direction::Up),
            "down" => Ok(Direction::Down),
            s => Err(format!("invalid direction: {}", s)),
        }
    }
}

#[derive(Debug, Default)]
struct Submarine {
    pub y: i64,
    pub z: i64,
    pub aim: i64,
}

impl Submarine {
    fn new(s: &str, with_gun: bool) -> Result<Self, String> {
        let mut sub = Submarine::default();

        for line in s.lines() {
            let mut pieces = line.split_whitespace();
            let direction = Direction::from_str(pieces.next().unwrap())?;
            let amount: i64 = pieces
                .next()
                .unwrap()
                .parse()
                .map_err(|e| format!("invalid amount: {:?}", e))?;
            match direction {
                Direction::Forward => {
                    sub.y += amount;

                    if with_gun {
                        sub.z += sub.aim * amount;
                    }
                }
                Direction::Up => {
                    if with_gun {
                        sub.aim -= amount;
                    } else {
                        sub.z -= amount;
                    }
                }
                Direction::Down => {
                    if with_gun {
                        sub.aim += amount;
                    } else {
                        sub.z += amount;
                    }
                }
            }
        }

        Ok(sub)
    }
}
