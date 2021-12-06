use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();

    let mut fish_counts: [usize; 9] = [0; 9];
    for fish in s.trim().split(',').map(|f| f.parse::<usize>().unwrap()) {
        fish_counts[fish] += 1;
    }

    for round in 0..256 {
        let mut total = 0;
        let zeroes = fish_counts[0];

        for i in 1..=8 {
            let num = fish_counts[i];
            fish_counts[i - 1] = num;
            total += num;
        }

        fish_counts[6] += zeroes;
        fish_counts[8] = zeroes;
        total += zeroes * 2;

        if round == 79 {
            println!("Part 1: {}", total);
        }

        if round == 255 {
            println!("Part 2: {}", total);
        }
    }
}
