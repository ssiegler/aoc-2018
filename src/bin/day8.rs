extern crate aoc_2018;

use aoc_2018::file_lines;

fn main() {
    let input: String = file_lines().collect();
    let mut iter = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let root = read_node(&mut iter);
    println!("Checksum: {}", root.meta_sum());
    println!("Value: {}", root.value());
}

struct Node {
    children: Vec<Node>,
    data: Vec<usize>,
}

impl Node {
    fn meta_sum(&self) -> usize {
        let child_sum: usize = self.children.iter().map(Node::meta_sum).sum();
        let meta_sum: usize = self.data.iter().sum();
        child_sum + meta_sum
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            self.data.iter().sum()
        } else {
            self.data.iter().filter_map(|&i| self.children.get(i-1).map(Node::value)).sum()
        }
    }
}

fn read_node(iter: &mut impl Iterator<Item=usize>) -> Node {
    let mut result = Node { children: Vec::new(), data: Vec::new() };
    let child_count = iter.next().unwrap();
    let meta_count = iter.next().unwrap();
    for _ in 0..child_count {
        result.children.push(read_node(iter));
    }
    for _ in 0..meta_count {
        result.data.push(iter.next().unwrap());
    }
    result
}