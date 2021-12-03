use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();
    let mut bits: Vec<Vec<u32>> = Vec::new();
    for (j, line) in s.lines().enumerate() {
        for (i, digit) in line.chars().enumerate() {
            if j == 0 {
                bits.push(Vec::new());
            }

            bits[i].push(digit.to_digit(10).unwrap());
        }
    }

    part1(&bits);
    part2(&bits);
}

fn part1(bits: &[Vec<u32>]) {
    let gamma = from_bits(bits.iter().map(|bits| {
        if bits.iter().copied().sum::<u32>() > (bits.len() as u32 / 2) {
            1u32
        } else {
            0u32
        }
    }));

    let mut epsilon = gamma;

    for i in 0..bits.len() {
        epsilon ^= 1 << i;
    }

    println!("Part 1: {}", gamma * epsilon);
}

fn from_bits<I>(bits: I) -> u32
where
    I: DoubleEndedIterator<Item = u32>,
{
    bits.rev()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * 2u32.pow(i as u32) as u32)
}

fn part2(bits: &[Vec<u32>]) {
    let mut oxygen_bits: Vec<u32> = Vec::new();
    let mut co2_bits: Vec<u32> = Vec::new();
    let mut oxygen_possibilities: Vec<usize> = (0..(bits[0].len())).collect();
    let mut co2_possibilities: Vec<usize> = (0..(bits[0].len())).collect();
    for i in 0..bits.len() {
        let mut skip_oxygen = false;
        if oxygen_possibilities.len() == 1 {
            oxygen_bits.push(bits[i][oxygen_possibilities[0]]);
            skip_oxygen = true;
        }
        if !skip_oxygen {
            let possible_oxygen_bits: Vec<u32> = bits[i]
                .iter()
                .enumerate()
                .filter_map(|(j, x)| {
                    if oxygen_possibilities.contains(&j) {
                        Some(*x)
                    } else {
                        None
                    }
                })
                .collect();
            let oxy_len = possible_oxygen_bits.len() as f32;
            let oxy_bit = if possible_oxygen_bits.into_iter().sum::<u32>() as f32 >= (oxy_len / 2.0)
            {
                1u32
            } else {
                0u32
            };
            oxygen_possibilities.retain(|j| bits[i][*j] == oxy_bit);
            oxygen_bits.push(oxy_bit);
        }

        if co2_possibilities.len() == 1 {
            co2_bits.push(bits[i][co2_possibilities[0]]);
            continue;
        }

        let possible_co2_bits: Vec<u32> = bits[i]
            .iter()
            .enumerate()
            .filter_map(|(j, x)| {
                if co2_possibilities.contains(&j) {
                    Some(*x)
                } else {
                    None
                }
            })
            .collect();

        let co2_len = possible_co2_bits.len() as f32;

        let co2_bit = if possible_co2_bits.into_iter().sum::<u32>() as f32 >= (co2_len / 2.0) {
            0u32
        } else {
            1u32
        };
        co2_possibilities.retain(|j| bits[i][*j] == co2_bit);
        co2_bits.push(co2_bit);
    }

    let oxygen_rating = from_bits(oxygen_bits.into_iter());
    let co2_rating = from_bits(co2_bits.into_iter());

    println!("Part 2: {}", oxygen_rating * co2_rating)
}
