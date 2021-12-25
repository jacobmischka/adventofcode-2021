use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();
    let mut floor: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect()).collect();

    let mut moved = true;
    let mut steps = 0;
    while moved {
        steps += 1;
        moved = false;

        let mut new_floor = floor.clone();

        for i in 0..floor.len() {
            for j in 0..floor[i].len() {
                let k = (j + 1) % floor[i].len();
                if floor[i][j] == '>' && floor[i][k] == '.' {
                    new_floor[i][j] = '.';
                    new_floor[i][k] = '>';
                    moved = true;
                }
            }
        }

        floor = new_floor.clone();

        for i in 0..floor.len() {
            let k = (i + 1) % floor.len();
            for j in 0..floor[i].len() {
                if floor[i][j] == 'v' && floor[k][j] == '.' {
                    new_floor[i][j] = '.';
                    new_floor[k][j] = 'v';
                    moved = true;
                }
            }
        }

        floor = new_floor;
        // println!("{}:", steps);
        // dump_floor(&floor);
    }

    println!("Part 1: {}", steps);
}

fn dump_floor(floor: &Vec<Vec<char>>) {
    for row in floor {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }

    println!();
}
