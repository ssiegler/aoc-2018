extern crate aoc_2018;

use aoc_2018::file_lines;

fn main() {
    let polymer = file_lines().collect();
    let reduced = reduce(polymer);
    println!("Remaining units: {}", reduced.len());
}

fn reduce(polymer: String) -> String {
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