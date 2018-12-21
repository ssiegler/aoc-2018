extern crate aoc_2018;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use aoc_2018::file_lines;

fn main() {
    let dependencies: Dependencies = file_lines().collect();
    println!("Order: {}", dependencies.measure(1, |_| 1).1);
    let (time, steps) = dependencies.measure(5, |id| id.0 as usize - 4);
    println!("Steps: {}, Order: {}", time, steps);
}

fn parse_line(line: &str) -> Option<(TaskId, TaskId)> {
    let mut chars = line.chars();
    Some((TaskId(chars.nth(5)?), TaskId(chars.nth(30)?)))
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct TaskId(char);

impl Ord for TaskId {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}

impl PartialOrd for TaskId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Dependencies(HashMap<TaskId, HashSet<TaskId>>);

impl Dependencies {
    fn get(&self, task: TaskId) -> Vec<&TaskId> {
        self.0.get(&task).map_or(vec![], |set| set.iter().collect())
    }

    fn count_required(&self) -> HashMap<TaskId, usize> {
        let mut result = HashMap::new();
        for &required in self.0.iter().flat_map(|(_key, value)| value.iter()) {
            let mut count = result.entry(required).or_insert(0);
            *count += 1;
        }
        result
    }

    fn find_available(&self, required: &HashMap<TaskId, usize>) -> BinaryHeap<TaskId> {
        self.0
            .keys()
            .chain(required.keys())
            .filter(|task| required.get(task).cloned().unwrap_or(0) == 0)
            .cloned()
            .collect()
    }

    fn order(&self) -> String {
        let mut required = self.count_required();
        let mut queue = self.find_available(&required);
        let mut result = String::new();
        while let Some(next) = queue.pop() {
            result.push(next.0);
            for task in self.get(next) {
                let count = required.get_mut(task).unwrap();
                *count -= 1;
                if *count == 0 {
                    queue.push(*task);
                }
            }
        }
        result
    }

    fn measure(&self, worker_count: usize, task_durations: fn(TaskId) -> usize) -> (usize, String) {
        let mut required = self.count_required();
        let mut queue = self.find_available(&required);
        let mut result = 0;
        let mut output = String::new();
        let mut workers: Vec<Option<(TaskId, usize)>> = vec![None; worker_count];

        while !queue.is_empty() || workers.iter().any(Option::is_some) {
            for mut worker in workers.iter_mut().filter(|v| v.is_none()) {
                if let Some(task) = queue.pop() {
                    worker.get_or_insert((task, task_durations(task)));
                }
            }

            if let Some((completed, elapsed)) = workers
                .iter()
                .filter_map(|v| *v)
                .min_by_key(|(_, remaining)| *remaining)
                .take()
                {
                    result += elapsed;
                    output.push(completed.0);
                    for mut worker in workers.iter_mut() {
                        if let Some((task, time)) = worker.take() {
                            let remaining = time - elapsed;
                            if remaining > 0 {
                                worker.replace((task, remaining));
                            }
                        }
                    }
                    for task in self.get(completed) {
                        let count = required.get_mut(task).unwrap();
                        *count -= 1;
                        if *count == 0 {
                            queue.push(*task);
                        }
                    }
                }

        }
        (result, output)
    }
}

impl<I> FromIterator<I> for Dependencies
    where
        I: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item=I>>(iter: T) -> Self {
        let mut depending = HashMap::new();
        for line in iter {
            if let Some((r, d)) = parse_line(line.as_ref()) {
                depending.entry(r).or_insert_with(HashSet::new).insert(d);
            }
        }
        Dependencies(depending)
    }
}
