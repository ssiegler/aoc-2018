extern crate aoc_2018;

use aoc_2018::file_lines;

fn main() {
    println!("Order: {}", order(file_lines()))
}

fn parse_line(line: &str) -> Option<(char, char)> {
    let mut chars = line.chars();
    Some((chars.nth(5)?, chars.nth(30)?))
}

fn order<I>(input: I) -> String
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    let mut result = String::new();
    for line in input {
        if let Some((required, depending)) = parse_line(line.as_ref()) {
            result.push(required);
            result.push(depending);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use std::iter::once;

    use super::*;

    #[test]
    fn no_tasks_without_dependencies() {
        assert_eq!("", order(once("")));
    }

    #[test]
    fn single_dependency() {
        assert_eq!(
            "AB",
            order(once("Step A must be finished before step B can begin."))
        );
        assert_eq!(
            "CD",
            order(once("Step C must be finished before step D can begin."))
        );
        assert_eq!(
            "CA",
            order(once("Step C must be finished before step A can begin."))
        );
    }

    #[test]
    fn two_separate_dependencies() {
        assert_eq!(
            "ABCD",
            order(
                [
                    "Step A must be finished before step B can begin.",
                    "Step C must be finished before step D can begin."
                ]
                .iter()
            )
        );
    }

    /*
        #[test]
        fn three_tasks() {
            assert_eq!("CAB", order("Step C must be finished before step B can begin.
    Step C must be finished before step A can begin."));
        }
        */
}
