extern crate aoc_2018;

use aoc_2018::file_lines;

fn main() {
    println!("Order: {}", order(file_lines()))
}

fn order<I>(input: I) -> String
where
    I: Iterator,
    I::Item: AsRef<str>,
{
    let mut result = String::new();
    for line in input {
        let mut chars = line.as_ref().chars();
        if let Some(required) = chars.nth(5) {
            result.push(required);
        }
        if let Some(depending) = chars.nth(30) {
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
