extern crate aoc_2018;

use std::collections::HashMap;
use std::collections::HashSet;
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

fn order_task_dependencies(dependencies: impl Iterator<Item = Dependency>) -> String {
    let mut tasks = Dependencies::new();
    for dependency in dependencies {
        tasks.add(dependency)
    }
    let order = tasks.resolve();
    order
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct TaskId(char);

#[derive(Debug, Eq, PartialEq)]
struct Dependency {
    depending: TaskId,
    required: TaskId,
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
                depending: TaskId(depending),
                required: TaskId(required),
            }),
            _ => Err(DependencyError::FormatError),
        }
    }
}

#[derive(Debug)]
struct Dependencies {
    tasks: HashMap<TaskId, Task>,
}

#[derive(Debug)]
struct Task {
    depending: HashSet<TaskId>,
    required: HashSet<TaskId>,
}

impl Task {
    fn new() -> Self {
        Task {
            depending: HashSet::new(),
            required: HashSet::new(),
        }
    }

    fn add_depending(&mut self, depending: TaskId) {
        self.depending.insert(depending);
    }

    fn add_required(&mut self, required: TaskId) {
        self.required.insert(required);
    }

    fn remove_required(&mut self, required: &TaskId) {
        self.required.remove(&required);
    }

    fn is_available(&self) -> bool {
        self.required.is_empty()
    }
}

impl Dependencies {
    fn new() -> Self {
        Dependencies {
            tasks: HashMap::new(),
        }
    }

    fn add(
        &mut self,
        Dependency {
            depending,
            required,
        }: Dependency,
    ) {
        self.tasks
            .entry(depending)
            .or_insert_with(|| Task::new())
            .add_required(required);
        self.tasks
            .entry(required)
            .or_insert_with(|| Task::new())
            .add_depending(depending);
    }

    fn complete(&mut self, completed: &TaskId) {
        if let Some(task) = self.tasks.remove(completed) {
            for depending in task.depending {
                if let Some(d) = self.tasks.get_mut(&depending) {
                    d.remove_required(completed);
                }
            }
        }
    }

    fn available(&self) -> Option<TaskId> {
        self.tasks
            .iter()
            .filter(|(_id, task)| task.is_available())
            .map(|(&id, _task)| id)
            .min()
    }

    fn resolve(&mut self) -> String {
        let mut result = String::new();
        while let Some(task) = self.available() {
            result.push(task.0);
            self.complete(&task);
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
                    required: TaskId('C'),
                    depending: TaskId('A'),
                },
                Dependency {
                    required: TaskId('C'),
                    depending: TaskId('F'),
                },
                Dependency {
                    required: TaskId('A'),
                    depending: TaskId('B'),
                },
                Dependency {
                    required: TaskId('A'),
                    depending: TaskId('D'),
                },
                Dependency {
                    required: TaskId('B'),
                    depending: TaskId('E'),
                },
                Dependency {
                    required: TaskId('D'),
                    depending: TaskId('E'),
                },
                Dependency {
                    required: TaskId('F'),
                    depending: TaskId('E'),
                },
            ]
        )
    }

    #[test]
    fn example_on_dependencies() {
        let dependencies = vec![
            Dependency {
                required: TaskId('C'),
                depending: TaskId('A'),
            },
            Dependency {
                required: TaskId('C'),
                depending: TaskId('F'),
            },
            Dependency {
                required: TaskId('A'),
                depending: TaskId('B'),
            },
            Dependency {
                required: TaskId('A'),
                depending: TaskId('D'),
            },
            Dependency {
                required: TaskId('B'),
                depending: TaskId('E'),
            },
            Dependency {
                required: TaskId('D'),
                depending: TaskId('E'),
            },
            Dependency {
                required: TaskId('F'),
                depending: TaskId('E'),
            },
        ];
        assert_eq!(order_task_dependencies(dependencies.into_iter()), "CABDFE");
    }
}
