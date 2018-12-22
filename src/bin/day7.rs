extern crate aoc_2018;

use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use aoc_2018::file_lines;

fn main() {
    let dependencies: Dependencies = file_lines().collect();
    println!(
        "Order: {}",
        dependencies.measure(&mut Workers::new(1, |_| 1)).tasks
    );
    let Plan { tasks, time } = dependencies.measure(&mut Workers::new(5, |id| id.0 as Seconds - 4));
    println!("Steps: {}, Order: {}", time, tasks);
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
    fn tasks(&self) -> impl Iterator<Item = (&TaskId, &HashSet<TaskId>)> {
        self.0.iter()
    }

    fn depending(&self, task: TaskId) -> Vec<&TaskId> {
        self.0.get(&task).map_or(vec![], |set| set.iter().collect())
    }

    fn measure(&self, workers: &mut Workers) -> Plan {
        let mut queue = Queue::from(&self);
        let mut result = Plan::new();

        while !queue.is_empty() || workers.busy() {
            workers.take_jobs(&mut queue);

            if let Some((completed, elapsed)) = workers.finish_job() {
                result.add(completed, elapsed);
                queue.insert(self.depending(completed).as_slice());
            }
        }
        result
    }
}

type Seconds = usize;

struct Workers {
    jobs: Vec<Option<(TaskId, Seconds)>>,
    durations: fn(TaskId) -> Seconds,
}

impl Workers {
    fn new(count: usize, durations: fn(TaskId) -> Seconds) -> Self {
        Workers {
            jobs: vec![None; count],
            durations,
        }
    }

    fn busy(&self) -> bool {
        self.jobs.iter().any(Option::is_some)
    }

    fn take_jobs(&mut self, queue: &mut Queue) {
        for mut worker in self.jobs.iter_mut().filter(|v| v.is_none()) {
            if let Some(task) = queue.pop() {
                worker.get_or_insert((task, (self.durations)(task)));
            }
        }
    }

    fn finish_job(&mut self) -> Option<(TaskId, Seconds)> {
        let job = self
            .jobs
            .iter()
            .filter_map(|v| *v)
            .min_by_key(|(_, remaining)| *remaining)
            .take();
        if let Some((_, elapsed)) = job {
            for mut worker in self.jobs.iter_mut() {
                if let Some((task, time)) = worker.take() {
                    let remaining = time - elapsed;
                    if remaining > 0 {
                        worker.replace((task, remaining));
                    }
                }
            }
        }
        job
    }
}

struct Plan {
    tasks: String,
    time: Seconds,
}

impl Plan {
    fn new() -> Self {
        Plan {
            tasks: String::new(),
            time: 0,
        }
    }

    fn add(&mut self, task: TaskId, time: Seconds) {
        self.tasks.push(task.0);
        self.time += time;
    }
}

struct Queue {
    required: HashMap<TaskId, usize>,
    available: BinaryHeap<TaskId>,
}

impl Queue {
    fn from(dependencies: &Dependencies) -> Self {
        let mut required = HashMap::new();
        for &task in dependencies.tasks().flat_map(|(_key, value)| value.iter()) {
            let mut count = required.entry(task).or_insert(0);
            *count += 1;
        }
        let available = dependencies
            .tasks()
            .map(|(task, _)| *task)
            .filter(|task| !required.contains_key(task))
            .collect();
        Queue {
            required,
            available,
        }
    }

    fn is_empty(&self) -> bool {
        self.available.is_empty()
    }

    fn pop(&mut self) -> Option<TaskId> {
        self.available.pop()
    }

    fn insert(&mut self, tasks: &[&TaskId]) {
        for &task in tasks {
            if let Some(mut count) = self.required.get_mut(task) {
                *count -= 1;
                if *count == 0 {
                    self.available.push(*task);
                }
            }
        }
    }
}

impl<I> FromIterator<I> for Dependencies
where
    I: AsRef<str>,
{
    fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
        let mut result = HashMap::new();
        for line in iter {
            if let Some((r, d)) = parse_line(line.as_ref()) {
                result.entry(r).or_insert_with(HashSet::new).insert(d);
                result.entry(d).or_insert_with(HashSet::new);
            }
        }
        Dependencies(result)
    }
}
