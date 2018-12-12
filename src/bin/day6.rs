extern crate aoc_2018;

use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use aoc_2018::file_lines;

fn main() {
    let sites = file_lines().map(|s| s.parse::<Point>().unwrap()).collect();

    println!("Largest finite area: {}", find_largest_finite_area(&sites));
}

fn find_largest_finite_area(sites: &Vec<Point>) -> u32 {
    let area = Area::from(sites);
    *area.measure_finite_areas().values().max().unwrap()
}

struct Area<'a> {
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
    sites: &'a Vec<Point>,
}

impl<'a> Area<'a> {
    fn from(sites: &'a Vec<Point>) -> Self {
        let mut min_x = u32::max_value();
        let mut max_x = u32::min_value();
        let mut min_y = u32::max_value();
        let mut max_y = u32::min_value();

        for site in sites {
            min_x = min_x.min(site.x);
            min_y = min_y.min(site.y);
            max_x = max_x.max(site.x);
            max_y = max_y.max(site.y);
        }
        Self { min_x, max_x, min_y, max_y, sites }
    }

    fn measure_inside(&self) -> HashMap<&Point, u32> {
        let mut finite_areas = HashMap::new();
        for x in self.min_x + 1..self.max_x {
            for y in self.min_y + 1..self.max_y {
                if let Some(site) = (Point { x, y }).find_unique_nearest_site(self.sites) {
                    let area = finite_areas.entry(site).or_insert(0);
                    *area += 1
                }
            }
        }
        finite_areas
    }

    fn measure_finite_areas(&self) -> HashMap<&Point, u32> {
        let mut finite_areas = self.measure_inside();
        self.remove_infinite_areas(&mut finite_areas);
        finite_areas
    }

    fn remove_infinite_areas(&self, finite_areas: &mut HashMap<&Point, u32>) {
        for x in self.min_x + 1..self.max_x {
            self.remove_nearest_site(&Point { x, y: self.min_y }, finite_areas);
            self.remove_nearest_site(&Point { x, y: self.max_y }, finite_areas);
        }
        for y in self.min_y + 1..self.max_y {
            self.remove_nearest_site(&Point { x: self.min_x, y }, finite_areas);
            self.remove_nearest_site(&Point { x: self.max_x, y }, finite_areas);
        }
    }

    fn remove_nearest_site(&self, point: &Point, finite_areas: &mut HashMap<&Point, u32>) {
        if let Some(site) = point.find_unique_nearest_site(self.sites) {
            finite_areas.remove(site);
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> u32 {
        let x_distance = if self.x > other.x { self.x - other.x } else { other.x - self.x };
        let y_distance = if self.y > other.y { self.y - other.y } else { other.y - self.y };
        x_distance + y_distance
    }

    fn find_unique_nearest_site<'a, 'b>(&'a self, sites: &'b Vec<Point>) -> Option<&'b Point> {
        let mut min = u32::max_value();
        let mut nearest = None;
        for site in sites {
            let distance = self.manhattan_distance(site);
            if distance < min {
                min = distance;
                nearest = Some(site);
            } else if distance == min {
                nearest = None;
            }
        }
        nearest
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let parts: Vec<&str> = s.splitn(2, ", ").collect();
        Ok(Point {
            x: parts[0].parse()?,
            y: parts[1].parse()?,
        })
    }
}