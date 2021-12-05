use adventofcode_2021::get_input;

use std::{collections::HashMap, str::FromStr};

fn main() {
    let s = get_input().unwrap();

    let lines: Vec<Line> = s
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect();

    let mut straight_intersections: HashMap<Point, usize> = HashMap::new();
    let mut intersections: HashMap<Point, usize> = HashMap::new();
    for line in &lines {
        let points = line.points();
        let straight = is_straight(&points).unwrap_or_default();
        for point in points {
            if straight {
                *straight_intersections.entry(point.clone()).or_default() += 1;
            }
            *intersections.entry(point).or_default() += 1;
        }
    }

    println!(
        "Part 1: {}",
        straight_intersections
            .values()
            .fold(0, |acc, x| if *x > 1 { acc + 1 } else { acc })
    );
    println!(
        "Part 2: {}",
        intersections
            .values()
            .fold(0, |acc, x| if *x > 1 { acc + 1 } else { acc })
    );
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point {
    x: u16,
    y: u16,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s.split(',');
        Ok(Point {
            x: coords
                .next()
                .and_then(|x| x.parse().ok())
                .ok_or_else(|| format!("invalid coordinates: {}", s))?,
            y: coords
                .next()
                .and_then(|x| x.parse().ok())
                .ok_or_else(|| format!("invalid coordinates: {}", s))?,
        })
    }
}

fn is_straight(points: &[Point]) -> Option<bool> {
    points.first().and_then(|first| {
        points
            .last()
            .map(|last| first.x == last.x || first.y == last.y)
    })
}

#[derive(Debug, Clone)]
struct Line(Point, Point);

impl Line {
    fn points(&self) -> Vec<Point> {
        let dx: i32 = self.1.x as i32 - self.0.x as i32;
        let dy: i32 = self.1.y as i32 - self.0.y as i32;

        let mut x = self.0.x;
        let mut y = self.0.y;

        let steps = dx.abs().max(dy.abs());

        (0..=steps)
            .map(|i| {
                let p = Point { x, y };

                if i < steps {
                    if dx > 0 {
                        x += 1;
                    } else if dx < 0 {
                        x -= 1;
                    }

                    if dy > 0 {
                        y += 1;
                    } else if dy < 0 {
                        y -= 1;
                    }
                }

                p
            })
            .collect()
    }
}

impl FromStr for Line {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split(" -> ");
        Ok(Line(
            Point::from_str(
                points
                    .next()
                    .ok_or_else(|| format!("missing point: {}", s))?,
            )?,
            Point::from_str(
                points
                    .next()
                    .ok_or_else(|| format!("missing point: {}", s))?,
            )?,
        ))
    }
}
