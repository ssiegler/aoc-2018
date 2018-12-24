use std::collections::VecDeque;

fn main() {
    println!("Score: {}", score(9, 25));
    println!("Score: {}", score(10, 1618));
    println!("Score: {}", score(13, 7999));
    println!("Score: {}", score(17, 1104));
    println!("Score: {}", score(21, 6111));
    println!("Score: {}", score(30, 5807));
    println!("Score: {}", score(428, 72061));
    println!("Score: {}", score(428, 7_206_100));
}

fn score(players: usize, rounds: usize) -> usize {
    let mut scores: Vec<usize> = vec![0; players];
    let mut player = 0;
    let mut game = Game::new();
    for round in 1..rounds {
        if let Some(score) = game.add(round) {
            scores[player] += score;
        }
        player = (player + 1) % scores.len()
    }
    *scores.iter().max().unwrap()
}

#[derive(Debug)]
struct Game {
    marbles: VecDeque<usize>,
}

impl Game {
    pub fn new() -> Self {
        let mut game = Game {
            marbles: VecDeque::new(),
        };
        game.marbles.push_front(0);
        game
    }

    pub fn add(&mut self, marble: usize) -> Option<usize> {
        if marble % 23 == 0 {
            for _ in 0..7 {
                self.move_counter_clockwise()
            }
            let result = self.marbles.pop_front().map(|removed| removed + marble);
            result
        } else {
            self.move_clockwise();
            self.move_clockwise();
            self.marbles.push_front(marble);
            None
        }
    }

    fn move_clockwise(&mut self) {
        if let Some(current) = self.marbles.pop_front() {
            self.marbles.push_back(current);
        }
    }

    fn move_counter_clockwise(&mut self) {
        if let Some(counter_clock_wise) = self.marbles.pop_back() {
            self.marbles.push_front(counter_clock_wise);
        }
    }
}
