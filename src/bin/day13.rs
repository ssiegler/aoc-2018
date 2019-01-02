extern crate aoc_2018;

use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fs::read_to_string;
use std::str::FromStr;

use aoc_2018::filename;

fn main() {
    let mut transport: Transport = read_to_string(filename())
        .expect("reading input failed")
        .parse()
        .expect("parsing transport failed");
    let mut no_crashes = true;
    while no_crashes {
        if let Some(crash) = transport.step().pop() {
            println!("First crash: {}", crash);
        }
    }
    while transport.carts.len() > 1 {
        transport.step();
    }
    println!(
        "Last cart is at: {}",
        transport.carts.first().unwrap().position
    );
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct Position {
    y: usize,
    x: usize,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

impl Position {
    fn step(&self, direction: &Direction) -> Position {
        use Direction::*;
        match direction {
            North => Position { x: self.x, y: self.y - 1 },
            East => Position { x: self.x + 1, y: self.y },
            South => Position { x: self.x, y: self.y + 1 },
            West => Position { x: self.x - 1, y: self.y },
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_char(ch: char) -> Option<Direction> {
        use Direction::*;
        match ch {
            '^' => Some(North),
            '>' => Some(East),
            'v' => Some(South),
            '<' => Some(West),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        use Direction::*;
        match self {
            North => '^',
            East => '>',
            South => 'v',
            West => '<',
        }
    }

    fn turn(&self, turn: &Turn) -> Direction {
        use Direction::*;
        use Turn::*;

        match (self, turn) {
            (North, Left) => West,
            (North, Straight) => North,
            (North, Right) => East,
            (East, Left) => North,
            (East, Straight) => East,
            (East, Right) => South,
            (South, Left) => East,
            (South, Straight) => South,
            (South, Right) => West,
            (West, Left) => South,
            (West, Straight) => West,
            (West, Right) => North,
        }
    }
}

#[derive(Debug)]
enum Track {
    Horizontal,
    Vertical,
    SlashCurve,
    BackslashCurve,
    Intersection,
}

impl Track {
    fn from_char(ch: char) -> Option<Track> {
        match ch {
            '-' | '<' | '>' => Some(Track::Horizontal),
            '|' | '^' | 'v' => Some(Track::Vertical),
            '/' => Some(Track::SlashCurve),
            '\\' => Some(Track::BackslashCurve),
            '+' => Some(Track::Intersection),
            _ => None,
        }
    }

    fn as_char(&self) -> char {
        use Track::*;
        match *self {
            Horizontal => '-',
            Vertical => '|',
            SlashCurve => '/',
            BackslashCurve => '\\',
            Intersection => '+',
        }
    }
}

#[derive(Debug)]
struct Cart {
    position: Position,
    facing: Direction,
    next_turn: Turn,
}

impl Cart {
    fn new(position: Position, facing: Direction) -> Self {
        Cart {
            position,
            facing,
            next_turn: Turn::Left,
        }
    }

    fn step(&mut self, track: &Track) {
        use Track::*;

        match track {
            Horizontal => Self::assert_horizontal(&self.facing),
            Vertical => Self::assert_vertical(&self.facing),
            SlashCurve => self.curve_slash(),
            BackslashCurve => self.curve_backslash(),
            Intersection => self.turn_on_intersection(),
        }
        self.position = self.position.step(&self.facing);
    }

    fn assert_vertical(direction: &Direction) {
        use Direction::*;
        match direction {
            East | West => panic!("Cannot go {:?} on a vertical track", direction),
            North | South => (),
        }
    }

    fn assert_horizontal(direction: &Direction) {
        use Direction::*;
        match direction {
            North | South => panic!("Cannot go {:?} on a horizontal track", direction),
            East | West => (),
        }
    }

    fn curve_slash(&mut self) {
        use Direction::*;
        self.facing = match self.facing {
            North => East,
            East => North,
            South => West,
            West => South,
        }
    }

    fn curve_backslash(&mut self) {
        use Direction::*;
        self.facing = match self.facing {
            North => West,
            East => South,
            South => East,
            West => North,
        }
    }

    fn turn_on_intersection(&mut self) {
        use Turn::*;

        self.facing = self.facing.turn(&self.next_turn);
        self.next_turn = match &self.next_turn {
            Left => Straight,
            Straight => Right,
            Right => Left,
        }
    }
}

#[derive(Debug)]
struct Transport {
    carts: Vec<Cart>,
    tracks: HashMap<Position, Track>,
    width: usize,
    height: usize,
}

impl Transport {
    fn step(&mut self) -> Vec<Position> {
        let mut crashes = HashMap::new();
        for mut cart in self.carts {
            if let Some(track) = self.tracks.get(&cart.position) {
                cart.step(track);
                crashes
                    .entry(&cart.position)
                    .and_modify(|count| *count += 1)
                    .or_insert(0);
            }
        }
        self.carts
            .retain(|cart| crashes.get(&cart.position).unwrap_or(&0) == &0);
        let mut crash_positions: Vec<Position> = crashes
            .iter()
            .filter_map(|(position, count)| if *count > 0 { Some(**position) } else { None })
            .collect();
        crash_positions.sort();
        crash_positions
    }

    fn find_cart(&self, position: &Position) -> Option<&Cart> {
        self.carts
            .binary_search_by_key(&position, |cart| &cart.position)
            .ok()
            .and_then(|index|self.carts.get(index))
    }
}

impl FromStr for Transport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut carts = Vec::new();
        let mut tracks = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        for (y, line) in s.lines().enumerate() {
            height += 1;
            width = width.max(line.len());
            for (x, cell) in line.char_indices() {
                let position = Position { x, y };
                if let Some(track) = Track::from_char(cell) {
                    tracks.insert(position, track);
                }
                if let Some(direction) = Direction::from_char(cell) {
                    carts.push(Cart::new(position, direction));
                }
            }
        }
        Ok(Transport {
            carts,
            tracks,
            width,
            height,
        })
    }
}

impl Display for Transport {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let position = Position { x, y };
                if let Some(cart) = self.find_cart(&position) {
                    write!(f, "{}", cart.facing.as_char())?;
                } else if let Some(track) = self.tracks.get(&position) {
                    write!(f, "{}", track.as_char())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
