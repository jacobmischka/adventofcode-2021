use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();
    let mut energies: Vec<Vec<u8>> = s
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let mut flashes = 0;
    let mut first_sync = None;
    let mut i = 0;
    while i < 100 || first_sync.is_none() {
        for row in &mut energies {
            for energy in row {
                *energy += 1;
            }
        }

        let mut flashers = Vec::new();
        for i in 0..energies.len() {
            for j in 0..energies[i].len() {
                if energies[i][j] > 9 {
                    flashers.push((i, j));
                }
            }
        }

        for (i, j) in flashers {
            flash(&mut energies, i, j);
        }

        let mut all_flashed = true;
        for row in &mut energies {
            for energy in row {
                if *energy > 9 {
                    *energy = 0;
                    flashes += 1;
                } else {
                    all_flashed = false;
                }
            }
        }

        if all_flashed && first_sync.is_none() {
            first_sync = Some(i + 1);
        }

        i += 1;
        if i == 100 {
            println!("Part 1: {}", flashes);
        }
    }

    println!("Part 2: {}", first_sync.unwrap());
}

fn flash(energies: &mut Vec<Vec<u8>>, i: usize, j: usize) {
    if i > 0 {
        energies[i - 1][j] += 1;
        if energies[i - 1][j] == 10 {
            flash(energies, i - 1, j);
        }
    }

    if i > 0 && j < energies[i - 1].len() - 1 {
        energies[i - 1][j + 1] += 1;
        if energies[i - 1][j + 1] == 10 {
            flash(energies, i - 1, j + 1);
        }
    }

    if j < energies[i].len() - 1 {
        energies[i][j + 1] += 1;
        if energies[i][j + 1] == 10 {
            flash(energies, i, j + 1);
        }
    }

    if i < energies.len() - 1 && j < energies[i + 1].len() - 1 {
        energies[i + 1][j + 1] += 1;
        if energies[i + 1][j + 1] == 10 {
            flash(energies, i + 1, j + 1);
        }
    }

    if i < energies.len() - 1 {
        energies[i + 1][j] += 1;
        if energies[i + 1][j] == 10 {
            flash(energies, i + 1, j);
        }
    }

    if i < energies.len() - 1 && j > 0 {
        energies[i + 1][j - 1] += 1;
        if energies[i + 1][j - 1] == 10 {
            flash(energies, i + 1, j - 1);
        }
    }

    if j > 0 {
        energies[i][j - 1] += 1;
        if energies[i][j - 1] == 10 {
            flash(energies, i, j - 1);
        }
    }

    if i > 0 && j > 0 {
        energies[i - 1][j - 1] += 1;
        if energies[i - 1][j - 1] == 10 {
            flash(energies, i - 1, j - 1);
        }
    }
}

#[allow(unused)]
fn dump_energies(energies: &Vec<Vec<u8>>) {
    for line in energies {
        for energy in line {
            print!("{}", energy);
        }
        println!()
    }

    println!("\n");
}
