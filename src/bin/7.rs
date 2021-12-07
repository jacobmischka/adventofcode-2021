use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();

    let mut positions: Vec<u32> = s.trim().split(',').map(|f| f.parse().unwrap()).collect();
    positions.sort();
    let med = positions[positions.len() / 2];
    let avg = positions.iter().copied().sum::<u32>() as f64 / positions.len() as f64;

    // Rounding isn't always right, just try them both
    let floor = avg.floor() as u32;
    let ceil = avg.ceil() as u32;

    let mut med_fuel = 0;
    let mut floor_fuel = 0;
    let mut ceil_fuel = 0;
    for pos in positions {
        med_fuel += pos.max(med) - pos.min(med);

        let d = pos.max(floor) - pos.min(floor);
        floor_fuel += (1..=d).into_iter().sum::<u32>();
        let d = pos.max(ceil) - pos.min(ceil);
        ceil_fuel += (1..=d).into_iter().sum::<u32>();
    }

    println!("Part 1: {}", med_fuel);
    println!("Part 2: {}", floor_fuel.min(ceil_fuel));
}
