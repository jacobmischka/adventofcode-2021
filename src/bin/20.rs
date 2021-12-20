use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();
    let mut lines = s.lines();
    let enhancer = to_bitvec(lines.next().unwrap());
    let mut input: Vec<Vec<u8>>;
    let mut output: Vec<Vec<u8>> = lines.skip(1).map(|line| to_bitvec(&line)).collect();

    let mut default = 0;
    for e in 0..50 {
        input = output;
        output = vec![vec![0; input[0].len() + 2]; input.len() + 2];

        for i in 0..output.len() {
            for j in 0..output[i].len() {
                let mut enhancement_index_bits = Vec::new();

                for k in (0..3).rev() {
                    let row = i.checked_sub(k).and_then(|index| input.get(index));
                    for l in (0..3).rev() {
                        enhancement_index_bits.push(
                            row.and_then(|row| j.checked_sub(l).and_then(|index| row.get(index)))
                                .copied()
                                .unwrap_or(default),
                        );
                    }
                }
                output[i][j] = enhancer[to_decimal(&enhancement_index_bits)];
            }
        }
        if enhancer[0] == 1 {
            default = if default == 1 { 0 } else { 1 };
        }

        if e == 1 {
            println!(
                "Part 1: {}",
                output.iter().fold(0u64, |acc, row| acc
                    + row.iter().map(|bit| *bit as u64).sum::<u64>())
            );
        }
    }

    println!(
        "Part 2: {}",
        output.iter().fold(0u64, |acc, row| acc
            + row.iter().map(|bit| *bit as u64).sum::<u64>())
    );
}

#[allow(unused)]
fn dump_image(image: &Vec<Vec<u8>>) {
    for i in 0..image.len() {
        for j in 0..image[i].len() {
            match image[i][j] {
                0 => print!("."),
                1 => print!("#"),
                x => panic!("invalid bit: {}", x),
            }
        }
        println!();
    }
}

fn to_decimal(bits: &[u8]) -> usize {
    let mut d = 0;

    for (i, b) in bits.iter().rev().enumerate() {
        d += *b as usize * 2usize.pow(i as u32)
    }

    d
}

fn to_bitvec(s: &str) -> Vec<u8> {
    s.chars()
        .map(|c| match c {
            '#' => 1,
            '.' => 0,
            _ => panic!(),
        })
        .collect()
}
