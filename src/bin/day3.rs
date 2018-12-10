extern crate aoc_2018;

use std::str::FromStr;
use aoc_2018::file_lines;

fn main() {
    println!("overlap area: {}", compute_overlap());
}

fn compute_overlap() -> usize {
    let claims: Vec<Rectangle> = file_lines().map(|s| s.parse::<Rectangle>().unwrap()).collect();
    let width: usize = claims.iter().map(|r| r.right() as usize).max().unwrap();
    let height: usize = claims.iter().map(|r| r.bottom() as usize).max().unwrap();
    let mut covered = vec![vec![0 as u32; width]; height];
    for claim in claims {
        claim.cover(&mut covered)
    }
    covered.iter().map(|ref v| v.iter().filter(|&&count| count > 1).count()).sum()
}

#[derive(Debug)]
struct Rectangle {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

impl Rectangle {
    fn right(&self) -> u32 {
        self.left + self.width
    }

    fn bottom(&self) -> u32 {
        self.top + self.height
    }

    fn cover(&self, area: &mut Vec<Vec<u32>>) {
        for x in self.left..self.right() {
            for y in self.top..self.bottom() {
                area[y as usize][x as usize] += 1;
            }
        }
    }
}

impl FromStr for Rectangle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s.splitn(2, " @ ").nth(1) {
            Some(coordinates) => {
                let mut parts = coordinates.splitn(5, |c: char| !c.is_numeric()).filter_map(|s| s.parse::<u32>().ok());
                match (parts.next(), parts.next(), parts.next(), parts.next()) {
                    (Some(left), Some(top), Some(width), Some(height)) => {
                        Ok(Rectangle { left, top, width, height })
                    },
                    _ => Err(format!("Invalid coordinates: {}", coordinates)),
                }
            },
            _ => Err(format!("Invalid claim: '{}'", s)),
        }
    }
}