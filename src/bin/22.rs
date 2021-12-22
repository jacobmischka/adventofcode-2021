use adventofcode_2021::get_input;

use std::ops;

fn main() {
    let s = get_input().unwrap();
    let mut ons: Vec<Cuboid> = Vec::new();

    for line in s.lines() {
        let (cuboid, on) = parse_step(&line);

        dedupe(&mut ons, cuboid);

        if on {
            ons.push(cuboid);
        }
    }

    let init_region = Cuboid {
        x: (-50, 50),
        y: (-50, 50),
        z: (-50, 50),
    };

    let mut on = 0;
    for c in &ons {
        if let Some(intersection) = c.intersection(&init_region) {
            on += intersection.volume();
        }
    }

    println!("Part 1: {}", on);
    println!(
        "Part 2: {}",
        ons.iter().fold(0, |acc, c| { acc + c.volume() })
    );
}

fn dedupe(cuboids: &mut Vec<Cuboid>, cuboid: Cuboid) {
    let mut i = 0;
    while i < cuboids.len() {
        if !cuboids[i].intersects(&cuboid) {
            i += 1;
            continue;
        }

        let prev = cuboids.remove(i);

        for c in prev - cuboid {
            assert!(c.x.0 <= c.x.1, "{:?}", c);
            assert!(c.y.0 <= c.y.1, "{:?}", c);
            assert!(c.z.0 <= c.z.1, "{:?}", c);

            cuboids.insert(i, c);
            i += 1;
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Cuboid {
    x: (isize, isize),
    y: (isize, isize),
    z: (isize, isize),
}

impl Cuboid {
    fn volume(&self) -> isize {
        ((self.x.1 - self.x.0).abs() + 1)
            * ((self.y.1 - self.y.0).abs() + 1)
            * ((self.z.1 - self.z.0).abs() + 1)
    }

    fn intersects(&self, rhs: &Cuboid) -> bool {
        self.x.0 <= rhs.x.1
            && self.x.1 >= rhs.x.0
            && self.y.0 <= rhs.y.1
            && self.y.1 >= rhs.y.0
            && self.z.0 <= rhs.z.1
            && self.z.1 >= rhs.z.0
    }

    fn intersection(&self, rhs: &Cuboid) -> Option<Cuboid> {
        if self.intersects(rhs) {
            Some(Cuboid {
                x: (self.x.0.max(rhs.x.0), self.x.1.min(rhs.x.1)),
                y: (self.y.0.max(rhs.y.0), self.y.1.min(rhs.y.1)),
                z: (self.z.0.max(rhs.z.0), self.z.1.min(rhs.z.1)),
            })
        } else {
            None
        }
    }
}

impl ops::Add<Cuboid> for Cuboid {
    type Output = Vec<Cuboid>;

    fn add(self, rhs: Cuboid) -> Self::Output {
        if let Some(intersection) = self.intersection(&rhs) {
            let mut v = rhs - intersection;
            v.push(self);
            v
        } else {
            vec![self, rhs]
        }
    }
}

impl ops::Sub<Cuboid> for Cuboid {
    type Output = Vec<Cuboid>;

    fn sub(self, rhs: Cuboid) -> Self::Output {
        if let Some(intersection) = self.intersection(&rhs) {
            let mut v = Vec::new();

            if self.x.0 < intersection.x.0 {
                let c = Cuboid {
                    x: (self.x.0, intersection.x.0 - 1),
                    y: self.y,
                    z: self.z,
                };
                dedupe(&mut v, c);
                v.push(c);
            }

            if self.x.1 > intersection.x.1 {
                let c = Cuboid {
                    x: (intersection.x.1 + 1, self.x.1),
                    y: self.y,
                    z: self.z,
                };
                dedupe(&mut v, c);
                v.push(c)
            }

            if self.y.0 < intersection.y.0 {
                let c = Cuboid {
                    x: self.x,
                    y: (self.y.0, intersection.y.0 - 1),
                    z: self.z,
                };

                dedupe(&mut v, c);
                v.push(c)
            }

            if self.y.1 > intersection.y.1 {
                let c = Cuboid {
                    x: self.x,
                    y: (intersection.y.1 + 1, self.y.1),
                    z: self.z,
                };
                dedupe(&mut v, c);
                v.push(c);
            }

            if self.z.0 < intersection.z.0 {
                let c = Cuboid {
                    x: self.x,
                    y: self.y,
                    z: (self.z.0, intersection.z.0 - 1),
                };
                dedupe(&mut v, c);
                v.push(c);
            }

            if self.z.1 > intersection.z.1 {
                let c = Cuboid {
                    x: self.x,
                    y: self.y,
                    z: (intersection.z.1 + 1, self.z.1),
                };
                dedupe(&mut v, c);
                v.push(c);
            }

            v
        } else {
            vec![self]
        }
    }
}

fn parse_step(s: &str) -> (Cuboid, bool) {
    let mut pieces = s.split_whitespace();
    let on = match pieces.next().unwrap() {
        "on" => true,
        "off" => false,
        _ => panic!(),
    };
    let mut pieces = pieces.next().unwrap().split(',');
    let mut range = pieces
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split("..");
    let x = (
        range.next().unwrap().parse::<isize>().unwrap(),
        range.next().unwrap().parse::<isize>().unwrap(),
    );
    let mut range = pieces
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split("..");
    let y = (
        range.next().unwrap().parse::<isize>().unwrap(),
        range.next().unwrap().parse::<isize>().unwrap(),
    );
    let mut range = pieces
        .next()
        .unwrap()
        .split('=')
        .nth(1)
        .unwrap()
        .split("..");
    let z = (
        range.next().unwrap().parse::<isize>().unwrap(),
        range.next().unwrap().parse::<isize>().unwrap(),
    );

    (Cuboid { x, y, z }, on)
}
