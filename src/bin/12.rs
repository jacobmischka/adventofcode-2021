use adventofcode_2021::get_input;

use std::collections::{HashMap, HashSet};

fn main() {
    let s = get_input().unwrap();
    let mut paths: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in s.lines() {
        let mut pieces = line.split('-');
        let start = pieces.next().unwrap();
        let end = pieces.next().unwrap();

        paths.entry(start).or_default().push(end);
        paths.entry(end).or_default().push(start);
    }

    println!("Part 1: {}", visit(&paths, Vec::new(), false, "start"));
    println!("Part 2: {}", visit(&paths, Vec::new(), true, "start"));
}

fn visit<'a>(
    paths: &HashMap<&'a str, Vec<&'a str>>,
    mut visited: Vec<&'a str>,
    small_double_visit_remaining: bool,
    cave: &'a str,
) -> usize {
    if cave == "end" {
        1
    } else {
        let connected = paths.get(&cave).unwrap();
        visited.push(cave);
        let mut ret = 0;
        for cave in connected {
            if cave.to_ascii_uppercase() == *cave
                || !visited.contains(cave)
                || small_double_visit_remaining && *cave != "start"
            {
                ret += visit(
                    paths,
                    visited.clone(),
                    small_double_visit_remaining
                        && (cave.to_ascii_uppercase() == *cave || !visited.contains(cave)),
                    cave,
                );
            }
        }
        ret
    }
}
