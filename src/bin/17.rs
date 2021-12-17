use adventofcode_2021::get_input;

use std::collections::HashSet;

fn main() {
    let s = get_input().unwrap();
    let target = parse_target_area(&s);

    let mut max_y = 0;
    let mut initials = HashSet::new();
    let mut dx = 300;
    while dx > -300 {
        let mut dy = 300; // Completely arbitrary
        while dy > -300 {
            if let Some((_, top)) = shoot(target, dx, dy) {
                max_y = max_y.max(top);
                initials.insert((dx, dy));
            }
            dy -= 1;
        }
        dx -= 1;
    }

    println!("Part 1: {}", max_y);
    println!("Part 2: {}", initials.len());
}

fn shoot(
    (target_x, target_y): ((i32, i32), (i32, i32)),
    mut dx: i32,
    mut dy: i32,
) -> Option<((i32, i32), i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;

    while y > target_y.0 {
        x += dx;
        y += dy;
        max_y = max_y.max(y);

        dy -= 1;
        if dx < 0 {
            dx += 1
        } else if dx > 0 {
            dx -= 1;
        }

        if target_x.0 <= x && x <= target_x.1 && target_y.0 <= y && y <= target_y.1 {
            return Some(((x, y), max_y));
        }
    }

    None
}

fn parse_target_area(s: &str) -> ((i32, i32), (i32, i32)) {
    let mut chunks = s.split_whitespace().skip(2);
    let x_chunk = chunks.next().unwrap();
    let y_chunk = chunks.next().unwrap();
    let is_sep = |c: char| matches!(c, '=' | '.' | ',');
    let mut x_bits = x_chunk.split(is_sep).skip(1);
    let mut y_bits = y_chunk.split(is_sep).skip(1);

    let x = (
        x_bits.next().unwrap().parse::<i32>().unwrap(),
        x_bits.skip(1).next().unwrap().parse::<i32>().unwrap(),
    );
    let y = (
        y_bits.next().unwrap().parse::<i32>().unwrap(),
        y_bits.skip(1).next().unwrap().parse::<i32>().unwrap(),
    );

    (x, y)
}
