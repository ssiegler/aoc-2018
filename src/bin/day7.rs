use std::collections::HashMap;
use std::collections::HashSet;

fn main() {

}

fn order(input: &str) -> String {
    let mut result = String::new();
    for line in input.lines() {
        let mut chars = line.chars();
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
    use super::*;

    #[test]
    fn no_tasks_without_dependencies() {
        assert_eq!("", order(""));
    }

    #[test]
    fn single_dependency() {
        assert_eq!(
            "AB",
            order("Step A must be finished before step B can begin.")
        );
        assert_eq!(
            "CD",
            order("Step C must be finished before step D can begin.")
        );
        assert_eq!(
            "CA",
            order("Step C must be finished before step A can begin.")
        );
    }

    #[test]
    fn two_separate_dependencies() {
        assert_eq!(
            "ABCD",
            order(
                "Step A must be finished before step B can begin.
Step C must be finished before step D can begin."
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
