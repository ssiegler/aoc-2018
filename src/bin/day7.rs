extern crate aoc_2018;

use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use aoc_2018::file_lines;

fn main() {

    println!("Steps: {}", order_tasks(file_lines()));
}

fn order_tasks (requirements: impl Iterator<Item=String>) -> String {
    let mut tasks = Dependencies::new();
    for requirement in requirements {
        let required = requirement.chars().nth(5).unwrap();
        let dependant = requirement.chars().nth(36).unwrap();
        tasks.add(required, dependant)
    }
    let order = tasks.resolve();
    order
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


#[cfg(test)]
mod tests {
    use super::*;
    use aoc_2018::file_lines_from;

    #[test]
    fn example_part1() {
        assert_eq!(order_tasks("Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.".lines().map(String::from)), "CABDFE");
    }

    #[test]
    fn part1() {
        assert_eq!(order_tasks(file_lines_from("day7-input.txt")), "BGJCNLQUYIFMOEZTADKSPVXRHW");
    }
}