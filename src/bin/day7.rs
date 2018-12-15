extern crate aoc_2018;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::str::FromStr;

use aoc_2018::file_lines;

fn main() {
    println!("Steps: {}", order_tasks(file_lines()));
}

fn order_tasks(input_lines: impl Iterator<Item = String>) -> String {
    order_task_dependencies(parse_dependencies(input_lines))
}

fn parse_dependencies(input: impl Iterator<Item = String>) -> impl Iterator<Item = Dependency> {
    input.map(|line| line.parse().unwrap())
}

fn order_task_dependencies(requirements: impl Iterator<Item = Dependency>) -> String {
    let mut tasks = Dependencies::new();
    for Dependency {
        depending,
        required,
    } in requirements
    {
        tasks.add(required, depending)
    }
    let order = tasks.resolve();
    order
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
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

#[derive(Debug, Eq, PartialEq)]
struct Dependency {
    depending: Task,
    required: Task,
}

#[derive(Debug)]
enum DependencyError {
    FormatError,
}

impl FromStr for Dependency {
    type Err = DependencyError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match (s.chars().nth(5), s.chars().nth(36)) {
            (Some(required), Some(depending)) => Ok(Dependency {
                depending: Task(depending),
                required: Task(required),
            }),
            _ => Err(DependencyError::FormatError),
        }
    }
}

#[derive(Debug)]
struct Dependencies {
    depending: HashMap<Task, Vec<Task>>,
}

impl Dependencies {
    fn new() -> Self {
        Dependencies {
            depending: HashMap::new(),
        }
    }

    fn add(&mut self, required: Task, depending: Task) {
        self.depending.entry(depending).or_insert_with(|| vec![]);
        self.depending
            .entry(required)
            .or_insert_with(|| vec![])
            .push(depending);
    }

    fn count_required(&self) -> HashMap<Task, usize> {
        let mut counts: HashMap<Task, usize> = HashMap::new();
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
        required_count
            .iter()
            .filter(|(_task, count)| **count == 0)
            .for_each(|(&task, _count)| queue.push(task));
        while let Some(task) = queue.pop() {
            result.push(task.0);
            for depending in self.depending.get(&task).unwrap() {
                let count = required_count.get_mut(&depending).unwrap();
                *count -= 1;
                if *count == 0 {
                    queue.push(*depending);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use aoc_2018::file_lines_from;

    use super::*;

    #[test]
    fn example_part1() {
        assert_eq!(
            order_tasks(
                "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
                    .lines()
                    .map(String::from)
            ),
            "CABDFE"
        );
    }

    #[test]
    fn part1() {
        assert_eq!(
            order_tasks(file_lines_from("day7-input.txt")),
            "BGJCNLQUYIFMOEZTADKSPVXRHW"
        );
    }

    #[test]
    fn dependencies_from_str() {
        let dependencies: Vec<Dependency> = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
            .lines()
            .map(Dependency::from_str)
            .filter_map(Result::ok)
            .collect();
        assert_eq!(
            dependencies,
            vec![
                Dependency {
                    required: Task('C'),
                    depending: Task('A')
                },
                Dependency {
                    required: Task('C'),
                    depending: Task('F')
                },
                Dependency {
                    required: Task('A'),
                    depending: Task('B')
                },
                Dependency {
                    required: Task('A'),
                    depending: Task('D')
                },
                Dependency {
                    required: Task('B'),
                    depending: Task('E')
                },
                Dependency {
                    required: Task('D'),
                    depending: Task('E')
                },
                Dependency {
                    required: Task('F'),
                    depending: Task('E')
                },
            ]
        )
    }

    #[test]
    fn example_on_dependencies() {
        let dependencies = vec![
            Dependency {
                required: Task('C'),
                depending: Task('A'),
            },
            Dependency {
                required: Task('C'),
                depending: Task('F'),
            },
            Dependency {
                required: Task('A'),
                depending: Task('B'),
            },
            Dependency {
                required: Task('A'),
                depending: Task('D'),
            },
            Dependency {
                required: Task('B'),
                depending: Task('E'),
            },
            Dependency {
                required: Task('D'),
                depending: Task('E'),
            },
            Dependency {
                required: Task('F'),
                depending: Task('E'),
            },
        ];
        assert_eq!(order_task_dependencies(dependencies.into_iter()), "CABDFE");
    }
}
