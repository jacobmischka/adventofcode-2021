use adventofcode_2021::get_input;

use std::collections::HashMap;

fn main() {
    let s = get_input().unwrap();
    let mut lines = s.lines();
    let polymer: Vec<char> = lines.next().unwrap().chars().collect();
    let mut pair_counts: HashMap<(char, char), usize> = HashMap::new();
    for pair in polymer.windows(2) {
        *pair_counts.entry((pair[0], pair[1])).or_default() += 1;
    }

    let insertion_rules: HashMap<(char, char), char> = lines
        .skip(1)
        .map(|line| {
            let mut pieces = line.split(" -> ");
            let mut inputs = pieces.next().unwrap().chars();
            (
                (inputs.next().unwrap(), inputs.next().unwrap()),
                pieces.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect();

    let mut new_pair_counts: HashMap<(char, char), usize>;
    for i in 0..40 {
        new_pair_counts = HashMap::new();

        for (pair, count) in pair_counts.into_iter() {
            let new_element = *insertion_rules.get(&pair).unwrap();
            *new_pair_counts.entry((pair.0, new_element)).or_default() += count;
            *new_pair_counts.entry((new_element, pair.1)).or_default() += count;
        }

        pair_counts = new_pair_counts;

        if i == 9 {
            println!("Part 1: {}", get_score(&polymer, &pair_counts));
        }
    }

    println!("Part 2: {}", get_score(&polymer, &pair_counts));
}

fn get_score(original_polymer: &Vec<char>, pair_counts: &HashMap<(char, char), usize>) -> usize {
    let mut element_counts: HashMap<char, usize> = HashMap::new();
    for (pair, count) in pair_counts.iter() {
        *element_counts.entry(pair.0).or_default() += count;
        *element_counts.entry(pair.1).or_default() += count;
    }
    *element_counts.entry(original_polymer[0]).or_default() += 1;
    *element_counts
        .entry(original_polymer[original_polymer.len() - 1])
        .or_default() += 1;
    let mut common_elements: Vec<(char, usize)> = element_counts
        .into_iter()
        .map(|(char, count)| (char, count / 2))
        .collect();
    common_elements.sort_by(|a, b| b.1.cmp(&a.1));
    common_elements[0].1 - common_elements[common_elements.len() - 1].1
}
