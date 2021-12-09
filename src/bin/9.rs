use adventofcode_2021::get_input;

use std::collections::HashSet;

fn main() {
    let s = get_input().unwrap();
    let heightmap: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut basins: Vec<usize> = Vec::new();
    let mut sum: u64 = 0;

    for y in 0..heightmap.len() {
        for x in 0..heightmap[y].len() {
            let mut is_min = true;

            let val = heightmap[y][x];

            if x > 0 && heightmap[y][x - 1] <= val {
                is_min = false;
            }

            if x < (heightmap[y].len() - 1) && heightmap[y][x + 1] <= val {
                is_min = false;
            }

            if y > 0 && heightmap[y - 1][x] <= val {
                is_min = false;
            }

            if y < (heightmap.len() - 1) && heightmap[y + 1][x] <= val {
                is_min = false;
            }

            if is_min {
                sum += val as u64 + 1;
                let mut basin = HashSet::new();
                basin.insert((y, x));
                basin_size(&heightmap, &mut basin, y, x);
                basins.push(basin.len());
            }
        }
    }

    basins.sort();
    println!("Part 1: {}", sum);
    println!(
        "Part 2: {}",
        basins.pop().unwrap() * basins.pop().unwrap() * basins.pop().unwrap()
    );
}

fn basin_size(heightmap: &Vec<Vec<u8>>, basin: &mut HashSet<(usize, usize)>, y: usize, x: usize) {
    if heightmap[y][x] == 9 {
        return;
    }

    basin.insert((y, x));

    if x > 0 && !basin.contains(&(y, x - 1)) {
        basin_size(heightmap, basin, y, x - 1);
    }

    if x < (heightmap[y].len() - 1) && !basin.contains(&(y, x + 1)) {
        basin_size(heightmap, basin, y, x + 1);
    }

    if y > 0 && !basin.contains(&(y - 1, x)) {
        basin_size(heightmap, basin, y - 1, x);
    }

    if y < (heightmap.len() - 1) && !basin.contains(&(y + 1, x)) {
        basin_size(heightmap, basin, y + 1, x);
    }
}
