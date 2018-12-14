extern crate aoc_2018;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use aoc_2018::file_lines;

fn main() {
    let mut tasks = Dependencies::new();
    for line in file_lines() {
        let required = line.chars().nth(5).unwrap();
        let dependant = line.chars().nth(36).unwrap();
        tasks.add(required, dependant)
    }

    println!("Steps: {}", tasks.resolve());
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Task(char);

impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Dependencies {
    depending: HashMap<char, Vec<char>>
}

impl Dependencies {
    fn new() -> Self {
        Dependencies {
            depending: HashMap::new(),
        }
    }

    fn add(&mut self, required: char, depending: char) {
        self.depending.entry(depending).or_insert_with(|| vec![]);
        self.depending.entry(required).or_insert_with(|| vec![]).push(depending);
    }

    fn count_required(&self) -> HashMap<char, usize> {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for (&required, depending) in &self.depending {
            counts.entry(required).or_insert(0);
            for &task in depending {
                *counts.entry(task).or_insert(0) += 1
            }
        }
        counts
    }

    fn resolve(&self) -> String {
        let mut result = String::new();
        let mut required_count = self.count_required();
        let mut queue = BinaryHeap::new();
        required_count.iter()
            .filter(|(_task, count)| **count == 0)
            .for_each(|(&task, _count)| queue.push(Task(task)));
        while let Some(Task(task)) = queue.pop() {
            result.push(task);
            for depending in self.depending.get(&task).unwrap() {
                let count = required_count.get_mut(depending).unwrap();
                *count -= 1;
                if *count == 0 {
                    queue.push(Task(*depending));
                }
            }
        }
        result
    }
}