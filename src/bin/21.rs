use adventofcode_2021::get_input;

use std::collections::HashMap;

const BOARD_MAX: u64 = 10;
const DIE_MAX: u64 = 100;

fn main() {
    let s = get_input().unwrap();
    let player_pos: Vec<u64> = s
        .lines()
        .map(|line| {
            line.split_whitespace()
                .rev()
                .next()
                .unwrap()
                .parse::<u64>()
                .unwrap()
                - 1
        })
        .collect();

    deterministic_die(player_pos.clone());
    // dirac_die(&player_pos);
}

fn dirac_die(starting_pos: &Vec<u64>) {
    // TODO: Need some way to tell when a player has lost
    let mut player_pos_counts: Vec<Vec<HashMap<u64, u64>>> = starting_pos
        .iter()
        .copied()
        .map(|pos| {
            let mut pos_counts = vec![HashMap::new(); BOARD_MAX as usize];
            pos_counts[pos as usize].insert(0, 1);
            pos_counts
        })
        .collect();

    let mut winning_games: Vec<u128> = Vec::new();
    let mut player_i = 0;
    loop {
        let orig = player_pos_counts[player_i];
        player_pos_counts[player_i] = vec![HashMap::new(); orig.len()];

        for i in 0..orig.len() {
            for (score, num_games) in orig[i].iter() {
                for roll in 1..=3 {
                    let new_pos = (i + roll) % player_pos_counts[player_i].len();
                    let new_score = *score + new_pos as u64 + 1;
                    if new_score >= 21 {
                        winning_games[player_i] += *num_games as u128;
                    } else {
                        *player_pos_counts[player_i][new_pos]
                            .entry(new_score)
                            .or_default() += num_games;
                    }
                }
            }
        }

        break;
    }

    let loser_i = (player_i + 1) % player_pos.len();

    println!("Part 1: {}", player_scores[loser_i] * num_rolls);
}

fn deterministic_die(mut player_pos: Vec<u64>) {
    let mut player_scores = vec![0u64; player_pos.len()];

    let mut player_i = 0;
    let mut roll: u64 = 0;
    let mut num_rolls: u64 = 0;
    loop {
        player_pos[player_i] = (player_pos[player_i] + ((roll + 2) * 3)) % BOARD_MAX;
        roll += 3 % DIE_MAX;
        num_rolls += 3;

        player_scores[player_i] += player_pos[player_i] + 1;

        if player_scores[player_i] >= 1000 {
            break;
        }

        player_i = (player_i + 1) % player_pos.len();
    }

    let loser_i = (player_i + 1) % player_pos.len();

    println!("Part 1: {}", player_scores[loser_i] * num_rolls);
}
