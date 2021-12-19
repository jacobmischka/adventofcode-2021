use adventofcode_2021::get_input;

use std::{collections::HashSet, fmt, ops, str::FromStr};

fn main() {
    let s = get_input().unwrap();
    let mut known_scanners: Vec<(Scanner, Vec<Coord>)> = Vec::new();
    let mut unknown_scanners: Vec<(Scanner, Vec<Coord>)> = Vec::new();
    let mut coords: Vec<Coord> = Vec::new();
    for line in s.lines() {
        if line.is_empty() {
            continue;
        } else if line.starts_with("---") {
            if !coords.is_empty() {
                if known_scanners.is_empty() {
                    known_scanners.push((Scanner::default(), coords));
                } else {
                    unknown_scanners.push((Scanner::default(), coords));
                }
                coords = Vec::new();
            }
        } else {
            coords.push(Coord::from_str(&line).unwrap());
        }
    }

    if !coords.is_empty() {
        unknown_scanners.push((Scanner::default(), coords));
    }

    unify(&mut known_scanners, &mut unknown_scanners);

    let mut beacons = HashSet::new();
    let mut max_dist = 0;
    for (scanner, coords) in &known_scanners {
        for coord in coords {
            beacons.insert(scanner.translate(*coord));
        }

        for (other, _) in &known_scanners {
            max_dist = max_dist.max(scanner.position.manhattan_distance(other.position));
        }
    }

    println!("Part 1: {}", beacons.len());
    println!("Part 2: {}", max_dist);
}

fn unify(
    known_scanners: &mut Vec<(Scanner, Vec<Coord>)>,
    unknown_scanners: &mut Vec<(Scanner, Vec<Coord>)>,
) {
    'outer: while !unknown_scanners.is_empty() {
        for u in 0..unknown_scanners.len() {
            for k in 0..known_scanners.len() {
                for ik in 0..known_scanners[k].1.len() {
                    for iu in 0..unknown_scanners[u].1.len() {
                        for rotation in Rotation::enumerate() {
                            for facing in Direction::enumerate() {
                                unknown_scanners[u].0.rotation = rotation;
                                unknown_scanners[u].0.facing = facing;

                                unknown_scanners[u].0.position = Coord::default();

                                unknown_scanners[u].0.position = known_scanners[k]
                                    .0
                                    .translate(known_scanners[k].1[ik])
                                    - unknown_scanners[u].0.translate(unknown_scanners[u].1[iu]);

                                assert_eq!(
                                    unknown_scanners[u].0.translate(unknown_scanners[u].1[iu]),
                                    known_scanners[k].0.translate(known_scanners[k].1[ik])
                                );

                                let mut in_common = HashSet::new();
                                'known_ref: for jk in 0..known_scanners[k].1.len() {
                                    for uk in 0..unknown_scanners[u].1.len() {
                                        if known_scanners[k].0.translate(known_scanners[k].1[jk])
                                            == unknown_scanners[u]
                                                .0
                                                .translate(unknown_scanners[u].1[uk])
                                        {
                                            in_common.insert(jk);
                                            continue 'known_ref;
                                        }
                                    }
                                }

                                if in_common.len() >= 12 {
                                    eprintln!("Locked in!");
                                    known_scanners.push(unknown_scanners.remove(u));
                                    continue 'outer;
                                }
                            }
                        }
                    }
                }
            }
        }

        panic!("Nothing new found :(");
    }

    assert!(unknown_scanners.is_empty());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    XPos,
    XNeg,
    YPos,
    YNeg,
    ZPos,
    ZNeg,
}

impl Direction {
    fn enumerate() -> [Direction; 6] {
        use Direction::*;

        [XPos, XNeg, YPos, YNeg, ZPos, ZNeg]
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::YPos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Zero,
    Ninety,
    OneEighty,
    TwoSeventy,
}

impl Rotation {
    fn enumerate() -> [Rotation; 4] {
        use Rotation::*;

        [Zero, Ninety, OneEighty, TwoSeventy]
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation::Zero
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Scanner {
    facing: Direction,
    rotation: Rotation,
    position: Coord,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Hash)]
struct Coord(i32, i32, i32);

impl Coord {
    fn manhattan_distance(self, rhs: Coord) -> u64 {
        (self.0 - rhs.0).abs() as u64
            + (self.1 - rhs.1).abs() as u64
            + (self.2 - rhs.2).abs() as u64
    }
}

impl ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.0, self.1, self.2)
    }
}

impl From<(i32, i32, i32)> for Coord {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Coord(x, y, z)
    }
}

impl FromStr for Coord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vals = s.split(',');
        Ok(Coord(
            vals.next()
                .ok_or_else(|| format!("missing X coord in {}", s))?
                .parse::<i32>()
                .map_err(|e| format!("invalid X coord in {}: {:?}", s, e))?,
            vals.next()
                .ok_or_else(|| format!("missing Y coord in {}", s))?
                .parse::<i32>()
                .map_err(|e| format!("invalid Y coord in {}: {:?}", s, e))?,
            vals.next()
                .ok_or_else(|| format!("missing Z coord in {}", s))?
                .parse::<i32>()
                .map_err(|e| format!("invalid Z coord in {}: {:?}", s, e))?,
        ))
    }
}

impl Scanner {
    fn translate(&self, mut pos: Coord) -> Coord {
        pos = match self.rotation {
            Rotation::Zero => pos,
            Rotation::Ninety => Coord(pos.2, pos.1, pos.0 * -1),
            Rotation::OneEighty => Coord(pos.0 * -1, pos.1, pos.2 * -1),
            Rotation::TwoSeventy => Coord(pos.2 * -1, pos.1, pos.0),
        };

        pos = match self.facing {
            Direction::XPos => Coord(pos.1, pos.0 * -1, pos.2),
            Direction::XNeg => Coord(pos.1 * -1, pos.0, pos.2),
            Direction::YPos => pos,
            Direction::YNeg => Coord(pos.0 * -1, pos.1 * -1, pos.2),
            Direction::ZPos => Coord(pos.0, pos.2 * -1, pos.1),
            Direction::ZNeg => Coord(pos.0, pos.2, pos.1 * -1),
        };

        pos + self.position
    }
}
