extern crate aoc_2018;
extern crate core;

use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;

use aoc_2018::file_lines;
use std::collections::VecDeque;

fn main() {
    let init = file_lines().next().expect("Invalid input");
    let rules: Rules = file_lines()
        .skip(2)
        .map(|line| line.parse::<Rule>().ok().unwrap())
        .collect();

    let mut state = Generation::new(init.split_at(15).1.to_string());

    for _ in 0..20 {
        state.apply(&rules);
    }

    println!("Generation {}: {}", state.number, state.value());

    let mut rates = VecDeque::new();

    while still_changing(&rates) {
        rates.push_back(state.rate(&rules));
        if rates.len() > 100 {
            rates.pop_front();
        }
    }
    let fixed_rate = rates.back().unwrap();

    println!(
        "Fixed rate of {} after {} generations",
        fixed_rate, state.number
    );
    println!(
        "50.000.000.000: {}",
        state.value() + (fixed_rate * (50_000_000_000 - state.number) as isize)
    );
}

fn still_changing(rates: &VecDeque<isize>) -> bool {
    rates.len() < 100
        || rates
            .front()
            .map_or(true, |last_rate| rates.iter().any(|rate| rate != last_rate))
}

#[derive(Debug)]
struct Generation {
    plants: String,
    offset: isize,
    number: u64,
}

impl Generation {
    pub fn new(plants: String) -> Self {
        Generation {
            plants,
            offset: 0,
            number: 0,
        }
    }

    fn rate(&mut self, rules: &Rules) -> isize {
        let last_value = self.value();
        self.apply(rules);
        self.value() - last_value
    }

    fn value(&self) -> isize {
        self.plants
            .chars()
            .enumerate()
            .map(|(i, ch)| {
                if ch == '#' {
                    i as isize + self.offset
                } else {
                    0
                }
            })
            .sum()
    }

    fn apply(&mut self, rules: &Rules) {
        let (offset, padded) = self.pad_state();
        self.offset += 2 + offset;
        self.plants = padded.windows(5).map(|w| rules.apply(w)).collect();
        self.number += 1;
    }

    fn pad_state(&self) -> (isize, Vec<u8>) {
        let front_dots = self.plants.find('#').unwrap_or(5);
        let mut padded: Vec<u8> = (front_dots..5).map(|_| b'.').collect();
        padded.extend(self.plants.bytes());
        while !padded.ends_with(b".....") {
            padded.push(b'.');
        }
        (front_dots as isize - 5, padded)
    }
}

#[derive(Debug)]
struct Rules {
    rules: HashMap<[u8; 5], char>,
}

impl Rules {
    fn new() -> Self {
        Rules {
            rules: HashMap::new(),
        }
    }

    fn add_rule(&mut self, rule: &Rule) {
        self.rules.insert(rule.pattern, rule.result);
    }

    fn apply(&self, pattern: &[u8]) -> char {
        *self.rules.get(pattern).unwrap_or(&'.')
    }
}

impl FromIterator<Rule> for Rules {
    fn from_iter<T: IntoIterator<Item = Rule>>(iter: T) -> Self {
        let mut rules = Rules::new();
        for rule in iter {
            rules.add_rule(&rule);
        }
        rules
    }
}

#[derive(Debug)]
struct Rule {
    pattern: [u8; 5],
    result: char,
}

struct RuleError {}

impl FromStr for Rule {
    type Err = RuleError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut rule = Rule {
            pattern: [b' '; 5],
            result: ' ',
        };
        for (i, b) in s.bytes().take(5).enumerate() {
            rule.pattern[i] = b;
        }
        rule.result = s.chars().nth(9).ok_or(RuleError {})?;
        Ok(rule)
    }
}
