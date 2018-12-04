extern crate aoc_2018;

use std::collections::HashSet;
use std::iter;

use aoc_2018::file_lines;

pub fn main() {
    println!("Frequency after one round: {}", sum_changes());
    println!("First frequency reached twice: {}", find_repeating_frequency());
}

fn sum_changes() -> i32 {
    let mut current_frequency: i32 = 0;
    for line in file_lines() {
        current_frequency += evaluate_line(&line);
    }
    current_frequency
}

fn evaluate_line(line: &str) -> i32 {
    let (sign, amount_str) = line.split_at(1);
    let amount: i32 = amount_str.parse().expect("unable to read amount");
    match sign {
        "+" => amount,
        "-" => -amount,
        _ => panic!("Unexpected sign '{}'", sign),
    }
}

fn find_repeating_frequency() -> i32 {
    let mut past_frequencies = HashSet::new();
    let mut current_frequency = 0;
    let mut lines = iter::repeat_with(|| file_lines()).flatten();
    while !past_frequencies.contains(&current_frequency) {
        past_frequencies.insert(current_frequency);
        current_frequency += evaluate_line(&(lines.next().unwrap()));
    }
    current_frequency
}
