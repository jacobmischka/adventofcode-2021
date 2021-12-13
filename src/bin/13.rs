use adventofcode_2021::get_input;

use std::collections::HashSet;

fn main() {
    let s = get_input().unwrap();

    let mut dots_section = true;
    let mut dots: HashSet<(usize, usize)> = HashSet::new();
    let mut num_instructions = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for line in s.lines() {
        if line.is_empty() {
            dots_section = false;
        } else if dots_section {
            let mut coords = line.split(',');
            dots.insert((
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            ));
        } else {
            let mut instruction = line.split_whitespace().nth(2).unwrap().split('=');
            let axis = instruction.next().unwrap();
            let val: usize = instruction.next().unwrap().parse().unwrap();

            match axis {
                "x" => {
                    max_x = val;
                    dots = dots
                        .into_iter()
                        .map(|(mut x, y)| {
                            if x > val {
                                x = val - (x - val);
                            }

                            (x, y)
                        })
                        .collect();
                }
                "y" => {
                    max_y = val;
                    dots = dots
                        .into_iter()
                        .map(|(x, mut y)| {
                            if y > val {
                                y = val - (y - val);
                            }

                            (x, y)
                        })
                        .collect();
                }
                _ => panic!(),
            }

            num_instructions += 1;
            if num_instructions == 1 {
                println!("Part 1: {}", dots.len());
            }
        }
    }

    println!("Part 2:\n");
    for y in 0..max_y {
        for x in 0..max_x {
            if dots.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}
