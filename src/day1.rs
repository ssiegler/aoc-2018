use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;
use std::iter;

pub fn main() {
    let filename = env::args().nth(1).expect("missing filename argument");
    println!("Frequency after one round: {}", sum_changes(&filename));
    println!("First frequency reached twice: {}", find_repeating_frequency(&filename));
}

fn sum_changes(filename: &String) -> i32 {
    let mut current_frequency: i32 = 0;
    let file = File::open(filename).expect("could not open file");
    for line in BufReader::new(file).lines().map(|line| line.unwrap()) {
        current_frequency += evaluate_line(&line);
    }
    current_frequency
}

fn evaluate_line(line: &String) -> i32 {
    let (sign, amount_str) = line.split_at(1);
    let amount: i32 = amount_str.parse().expect("unable to read amount");
    match sign {
        "+" => amount,
        "-" => -amount,
        _ => panic!("Unexpected sign '{}'", sign),
    }
}

fn find_repeating_frequency(filename: &String) -> i32 {
    let mut past_frequencies = HashSet::new();
    let mut current_frequency = 0;
    let mut lines =
        iter::repeat_with(|| File::open(filename).expect("could not open file"))
            .flat_map(|file| BufReader::new(file).lines()
                .map(|line| line.unwrap()));
    while !past_frequencies.contains(&current_frequency) {
        past_frequencies.insert(current_frequency);
        current_frequency += evaluate_line(&(lines.next().unwrap()));
    }
    current_frequency
}
