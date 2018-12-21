extern crate aoc_2018;

use aoc_2018::file_lines;

fn main() {
    let polymer: String = file_lines().collect();
    println!("Remaining units: {}", reduce(polymer.as_str()).len());
    println!(
        "Shortest polymer length: {}",
        find_shortest_variant_length(polymer.as_str())
    );
}

fn reduce(polymer: &str) -> String {
    let mut result = String::new();
    for unit in polymer.chars() {
        add_unit_with_reaction(&mut result, unit);
    }
    result
}

fn add_unit_with_reaction(result: &mut String, unit: char) {
    if let Some(last) = result.chars().last() {
        if can_react(last, unit) {
            result.pop();
            return;
        }
    }
    result.push(unit);
}

fn can_react(a: char, b: char) -> bool {
    a != b && a.eq_ignore_ascii_case(&b)
}

fn find_shortest_variant_length(polymer: &str) -> usize {
    (b'a'..=b'z')
        .map(char::from)
        .map(|unit| reduce(remove_unit(polymer, unit).as_str()).len())
        .min()
        .unwrap()
}

fn remove_unit(polymer: &str, unit: char) -> String {
    polymer.replace(|ch: char| ch.eq_ignore_ascii_case(&unit), "")
}
