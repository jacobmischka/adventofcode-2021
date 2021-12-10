use adventofcode_2021::get_input;

fn main() {
    let s = get_input().unwrap();

    let mut corrupt_score = 0;
    let mut incomplete_scores = Vec::new();
    for line in s.lines() {
        let mut stack = Vec::new();
        let mut corrupt = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => {
                    stack.push(c);
                }
                ')' => {
                    let left = stack.pop();
                    if left != Some('(') {
                        if left.is_some() {
                            corrupt_score += 3;
                            corrupt = true;
                        }
                        break;
                    }
                }
                ']' => {
                    let left = stack.pop();

                    if left != Some('[') {
                        if left.is_some() {
                            corrupt_score += 57;
                            corrupt = true;
                        }
                        break;
                    }
                }
                '}' => {
                    let left = stack.pop();
                    if left != Some('{') {
                        if left.is_some() {
                            corrupt_score += 1197;
                            corrupt = true;
                        }
                        break;
                    }
                }
                '>' => {
                    let left = stack.pop();
                    if left != Some('<') {
                        if left.is_some() {
                            corrupt_score += 25137;
                            corrupt = true;
                        }
                        break;
                    }
                }
                _ => panic!(),
            }
        }

        if !corrupt {
            let mut line_score: u64 = 0;
            for remaining in stack.into_iter().rev() {
                line_score *= 5;
                line_score += match remaining {
                    '(' => 1,
                    '[' => 2,
                    '{' => 3,
                    '<' => 4,
                    _ => panic!(),
                };
            }
            incomplete_scores.push(line_score);
        }
    }

    incomplete_scores.sort();
    let incomplete_score = incomplete_scores[incomplete_scores.len() / 2];

    println!("Part 1: {}", corrupt_score);
    println!("Part 2: {}", incomplete_score);
}
