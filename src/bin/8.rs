use adventofcode_2021::get_input;

use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    ops::Deref,
    str::FromStr,
};

fn main() {
    let s = get_input().unwrap();
    let mut inputs: Vec<Vec<Digit>> = Vec::new();
    let mut outputs: Vec<Vec<Digit>> = Vec::new();
    for line in s.lines() {
        let mut chunks = line.split(" | ");
        inputs.push(
            chunks
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| Digit::from_str(s).unwrap())
                .collect(),
        );
        outputs.push(
            chunks
                .next()
                .unwrap()
                .split_whitespace()
                .map(|s| Digit::from_str(s).unwrap())
                .collect(),
        );
    }

    let mut outputs = outputs.into_iter();
    let mut sum = 0;
    let mut one_four_seven_or_eight = 0;

    for line in inputs.into_iter() {
        let mut digits = HashMap::new();
        let mut remaining_digits = Vec::new();
        for digit in line.into_iter() {
            if let Some(value) = digit.value() {
                digits.insert(value, digit);
            } else {
                remaining_digits.push(digit);
            }
        }

        let mut possible_values: HashSet<u8> = vec![0, 2, 3, 5, 6, 9].into_iter().collect();
        while possible_values.len() > 0 {
            for i in 0..remaining_digits.len() {
                let digit = &remaining_digits[i];
                if possible_values.len() == 1 {
                    digits.insert(
                        *possible_values.iter().next().unwrap(),
                        remaining_digits.remove(i),
                    );
                    possible_values.clear();
                    break;
                }

                if digit.len() == 6 {
                    if digits.get(&4).unwrap().intersection(&digit).count() == 4
                        && possible_values.contains(&9)
                    {
                        digits.insert(9, remaining_digits.remove(i));
                        possible_values.remove(&9);
                        break;
                    }

                    if digits.get(&7).unwrap().intersection(&digit).count() == 3 {
                        if possible_values.contains(&0) {
                            digits.insert(0, remaining_digits.remove(i));
                            possible_values.remove(&0);
                            break;
                        }
                    }
                    if digits.get(&4).unwrap().intersection(&digit).count() == 3 {
                        let val = digit.len() as u8;
                        if possible_values.contains(&val) {
                            digits.insert(val, remaining_digits.remove(i));
                            possible_values.remove(&val);
                            break;
                        }
                    }
                } else if digit.len() == 5 {
                    if digits.get(&7).unwrap().intersection(&digit).count() == 3 {
                        if possible_values.contains(&3) {
                            digits.insert(3, remaining_digits.remove(i));
                            possible_values.remove(&3);
                            break;
                        }

                        if digits.get(&1).unwrap().intersection(&digit).count() == 2
                            && possible_values.contains(&3)
                        {
                            digits.insert(3, remaining_digits.remove(i));
                            possible_values.remove(&3);
                            break;
                        }
                    }

                    if digits.get(&4).unwrap().intersection(&digit).count() == 3 {
                        let val = digit.len() as u8;
                        if possible_values.contains(&val) {
                            digits.insert(val, remaining_digits.remove(i));
                            possible_values.remove(&val);
                            break;
                        }
                    }

                    if possible_values.contains(&2) {
                        digits.insert(2, remaining_digits.remove(i));
                        possible_values.remove(&2);
                        break;
                    }
                }
            }
        }

        let output = outputs.next().unwrap();
        let mut current_output = 0;

        'outer: for (i, d) in output.into_iter().rev().enumerate() {
            if d.value().is_some() {
                one_four_seven_or_eight += 1;
            }

            for val in 0..=9 {
                if let Some(digit) = digits.get(&val) {
                    if digit.intersection(&d).count() == digit.len() && digit.len() == d.len() {
                        current_output += val as u64 * 10u64.pow(i as u32);
                        continue 'outer;
                    }
                }
            }
        }

        sum += current_output;
    }

    println!("Part 1: {}", one_four_seven_or_eight);
    println!("Part 2: {}", sum);
}

#[derive(Debug)]
struct Digit(HashSet<char>);

impl FromStr for Digit {
    type Err = Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Digit(s.chars().collect()))
    }
}

impl Deref for Digit {
    type Target = HashSet<char>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Digit {
    fn value(&self) -> Option<u8> {
        match self.0.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            7 => Some(8),
            _ => None,
        }
    }

    fn possible_values(&self) -> Vec<u8> {
        let value = self.value();
        if let Some(value) = value {
            return vec![value];
        }

        match self.0.len() {
            5 => vec![2, 3, 5],
            6 => vec![0, 6, 9],
            _ => panic!(),
        }
    }
}
