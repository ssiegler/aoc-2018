extern crate aoc_2018;

use std::num::ParseIntError;
use std::str::FromStr;

use aoc_2018::file_lines;

fn main() {
    let mut stars: Vec<Star> = file_lines()
        .map(|line| line.parse::<Star>().expect("Invalid input"))
        .collect();
    let mut grid = Grid {
        stars: stars.as_mut_slice(),
    };

    let mut area = usize::max_value();
    let mut count = 0;
    while grid.area() < area  {
        area = grid.area();
        grid.step();
        count += 1;
    }
    grid.back();
    println!("{}\n{}", count-1, grid.print());
}

struct Grid<'a> {
    stars: &'a mut [Star],
}

impl<'a> Grid<'a> {
    fn area(&self) -> usize {
        let y = (self.max_y() - self.min_y()) as usize;
        let x = (self.max_x() - self.min_x()) as usize;
        x * y
    }

    fn print(&self) -> String {
        let mut output = String::new();
        for y in self.min_y()..=self.max_y() {
            for x in self.min_x()..=self.max_x() {
                output.push(if self.has_star(x, y) { '#' } else { '.' });
            }
            output.push('\n');
        }
        output
    }

    fn step(&mut self) {
        self.stars.iter_mut().for_each(Star::step);
    }

    fn back(&mut self) {
        self.stars.iter_mut().for_each(Star::back);
    }

    fn min_x(&self) -> isize {
        self.stars.iter().map(|star| star.x).min().unwrap()
    }

    fn min_y(&self) -> isize {
        self.stars.iter().map(|star| star.y).min().unwrap()
    }

    fn max_x(&self) -> isize {
        self.stars.iter().map(|star| star.x).max().unwrap()
    }

    fn max_y(&self) -> isize {
        self.stars.iter().map(|star| star.y).max().unwrap()
    }

    fn has_star(&self, x: isize, y: isize) -> bool {
        self.stars.iter().any(|star| star.x == x && star.y == y)
    }
}

struct Star {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

impl Star {
    fn step(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }

    fn back(&mut self) {
        self.x -= self.vx;
        self.y -= self.vy;
    }
}

#[derive(Debug)]
enum FormatError {
    ParseError(ParseIntError),
    MissingError,
}

impl From<ParseIntError> for FormatError {
    fn from(error: ParseIntError) -> Self {
        FormatError::ParseError(error)
    }
}

impl FromStr for Star {
    type Err = FormatError;

    fn from_str(s: &str) -> Result<Self, FormatError> {
        let parts: Vec<isize> = s
            .split(&['<', ',', '>'][..])
            .enumerate()
            .filter_map(|(i, e)| if i == 0 || i == 3 { None } else { Some(e) })
            .take(4)
            .map(|part: &str| part.trim().parse::<isize>())
            .collect::<Result<Vec<isize>, _>>()?;
        let &x = parts.get(0).ok_or(FormatError::MissingError)?;
        let &y = parts.get(1).ok_or(FormatError::MissingError)?;
        let &vx = parts.get(2).ok_or(FormatError::MissingError)?;
        let &vy = parts.get(3).ok_or(FormatError::MissingError)?;
        Ok(Star { x, y, vx, vy })
    }
}
