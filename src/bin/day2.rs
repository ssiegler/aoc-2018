extern crate aoc_2018;

use std::collections::HashMap;
use aoc_2018::file_lines;

pub fn main() {
    println!("Checksum: {}", checksum());
    println!("Common letters: {}", find_common_letters());
}

fn checksum() -> u32 {
    let mut twos = 0;
    let mut threes = 0;
    for counts in file_lines().map(|id| count_letters(&id)) {
        if counts.values().any(|&count| count == 2) {
            twos += 1;
        }
        if counts.values().any(|&count| count == 3) {
            threes += 1;
        }
    }
    twos * threes
}

fn count_letters(id: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for letter in id.chars() {
        let count = counts.entry(letter).or_insert(0);
        *count += 1;
    }
    counts
}

fn find_common_letters() -> String {
    let ids: Vec<String> = file_lines().collect();
    for (i, el1) in ids.iter().enumerate() {
        for el2 in ids.iter().skip(i + 1) {
            if different_chars_count(el1, el2) == 1 {
                return same_chars(el1, el2);
            }
        }
    };
    String::new()
}

fn different_chars_count(str1: &str, str2: &str) -> usize {
    str1.chars().zip(str2.chars())
        .filter(|(ch1, ch2)| ch1 != ch2)
        .count()
}

fn same_chars(str1: &str, str2: &str) -> String {
    str1.chars().zip(str2.chars())
        .filter_map(|(ch1, ch2)| if ch1 == ch2 { Some(ch1) } else { None })
        .collect()
}
