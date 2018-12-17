extern crate aoc_2018;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use aoc_2018::file_lines;
use std::iter::FromIterator;

fn main() {
    let dependencies = parse_dependencies(file_lines());
    println!("Steps: {}", order_tasks(dependencies));
}

fn parse_dependencies<I>(input: I) -> Vec<Dependency>
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    input.map(|line| line.as_ref().parse().unwrap()).collect()
}

fn order_tasks(dependencies: Vec<Dependency>) -> String {
    let mut tasks: Dependencies = dependencies.into_iter().collect();
    tasks.resolve()
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

#[derive(Debug)]
struct Dependencies {
    tasks: HashMap<TaskId, Task>,
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

impl FromIterator<Dependency> for Dependencies {
    fn from_iter<T: IntoIterator<Item=Dependency>>(iter: T) -> Self {
        let mut dependencies = Dependencies::new();
        for dependency in iter {
            dependencies.add(dependency)
        }
        dependencies
    }
}

#[cfg(test)]
mod tests {
    use aoc_2018::file_lines_from;

    use super::*;

    #[test]
    fn example_part1() {
        let dependencies = parse_dependencies(
            "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin."
                .lines(),
        );
        assert_eq!(order_tasks(dependencies), "CABDFE");
    }

    #[test]
    fn part1() {
        let dependencies = parse_dependencies(file_lines_from("day7-input.txt"));
        assert_eq!(order_tasks(dependencies), "BGJCNLQUYIFMOEZTADKSPVXRHW");
    }

    #[test]
    fn dependency_from_str() {
        assert_eq!(
            Dependency::from_str("Step C must be finished before step A can begin.").unwrap(),
            Dependency {
                required: TaskId('C'),
                depending: TaskId('A'),
            }
        );
        assert_eq!(
            Dependency::from_str("Step C must be finished before step F can begin.").unwrap(),
            Dependency {
                required: TaskId('C'),
                depending: TaskId('F'),
            }
        );
        assert_eq!(
            Dependency::from_str("Step A must be finished before step B can begin.").unwrap(),
            Dependency {
                required: TaskId('A'),
                depending: TaskId('B'),
            }
        );
        assert_eq!(
            Dependency::from_str("Step A must be finished before step D can begin.").unwrap(),
            Dependency {
                required: TaskId('A'),
                depending: TaskId('D'),
            }
        );
        assert_eq!(
            Dependency::from_str("Step B must be finished before step E can begin.").unwrap(),
            Dependency {
                required: TaskId('B'),
                depending: TaskId('E'),
            }
        );
        assert_eq!(
            Dependency::from_str("Step D must be finished before step E can begin.").unwrap(),
            Dependency {
                required: TaskId('D'),
                depending: TaskId('E'),
            }
        );
        assert_eq!(
            Dependency::from_str("Step F must be finished before step E can begin.").unwrap(),
            Dependency {
                required: TaskId('F'),
                depending: TaskId('E'),
            }
        );
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
        assert_eq!(order_tasks(dependencies), "CABDFE");
    }
}
