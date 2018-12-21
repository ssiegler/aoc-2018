extern crate aoc_2018;
extern crate core;

use aoc_2018::file_lines;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let sleep_times = read_sleep_times();

    let (&Guard(guard), histogram) = sleep_times.iter().max_by_key(|(_k, v)| v.sum()).unwrap();
    let minute = u32::from(histogram.mode());
    println!(
        "Choosing guard {} and minute {}: {}",
        guard,
        minute,
        guard * minute
    );
    let (&Guard(guard), histogram) = sleep_times
        .iter()
        .max_by_key(|(_k, v)| v.max_freq())
        .unwrap();
    let minute = u32::from(histogram.mode());
    println!(
        "Choosing guard {} and minute {}: {}",
        guard,
        minute,
        guard * minute
    );
}

fn read_sleep_times() -> HashMap<Guard, Histogram> {
    let mut entries: Vec<LogEntry> = file_lines()
        .filter_map(|line| LogEntry::from_str(line.as_str()).ok())
        .collect();
    entries.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));
    let mut result: HashMap<Guard, Histogram> = HashMap::new();
    let mut guard: Guard = Guard(0);
    let mut start: u8 = 0;
    for entry in entries {
        match entry.action {
            Action::BeginsShift(next_guard) => {
                guard = next_guard;
            }
            Action::FallsAsleep => start = entry.timestamp.minute,
            Action::WakesUp => result
                .entry(guard)
                .or_insert_with(Histogram::new)
                .add(start, entry.timestamp.minute),
        }
    }
    result
}

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Guard(u32);

struct Histogram([u32; 60]);

impl Histogram {
    fn new() -> Histogram {
        Histogram([0; 60])
    }

    fn add(&mut self, start: u8, end: u8) {
        for i in start..end {
            self.0[i as usize] += 1;
        }
    }

    fn sum(&self) -> u32 {
        self.0.iter().sum()
    }

    fn max_freq(&self) -> u32 {
        *self.0.iter().max().unwrap()
    }

    fn mode(&self) -> u8 {
        self.0
            .iter()
            .enumerate()
            .max_by_key(|(_minute, &count)| count)
            .map(|(minute, _)| minute as u8)
            .unwrap()
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Timestamp {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
}

impl FromStr for Timestamp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(Timestamp {
            year: s[0..4].parse()?,
            month: s[5..7].parse()?,
            day: s[8..10].parse()?,
            hour: s[11..13].parse()?,
            minute: s[14..16].parse()?,
        })
    }
}

enum Action {
    BeginsShift(Guard),
    FallsAsleep,
    WakesUp,
}

impl FromStr for Action {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        Ok(match s {
            "falls asleep" => Action::FallsAsleep,
            "wakes up" => Action::WakesUp,
            _ => Action::BeginsShift(Guard(s[7..].splitn(2, ' ').next().unwrap().parse()?)),
        })
    }
}

struct LogEntry {
    timestamp: Timestamp,
    action: Action,
}

impl FromStr for LogEntry {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let timestamp = s[1..17].parse::<Timestamp>()?;
        let action = s[19..].parse::<Action>()?;
        Ok(LogEntry { timestamp, action })
    }
}
