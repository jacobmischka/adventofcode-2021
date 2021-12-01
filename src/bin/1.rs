use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();
    let measurements = parse_measurements(&s);
    println!("Part 1: {}", num_increases(&measurements));
    println!("Part 2: {}", window_increases(&measurements));
}

fn window_increases(measurements: &[u32]) -> usize {
    measurements
        .windows(3)
        .fold((0, None), |(increases, prev), val| {
            let sum: u32 = val.iter().copied().sum();
            match prev {
                None => (increases, Some(sum)),
                Some(prev) => (
                    if sum > prev { increases + 1 } else { increases },
                    Some(sum),
                ),
            }
        })
        .0
}

fn num_increases(measurements: &[u32]) -> usize {
    measurements
        .iter()
        .copied()
        .fold((0, None), |(increases, prev), val| match prev {
            None => (increases, Some(val)),
            Some(prev) => (
                if val > prev { increases + 1 } else { increases },
                Some(val),
            ),
        })
        .0
}

fn parse_measurements(s: &str) -> Vec<u32> {
    s.lines().map(|line| line.parse::<u32>().unwrap()).collect()
}
