fn main() {
    println!("Score: {}", score(9, 25));
    println!("Score: {}", score(10, 1618));
    println!("Score: {}", score(13, 7999));
    println!("Score: {}", score(17, 1104));
    println!("Score: {}", score(21, 6111));
    println!("Score: {}", score(30, 5807));
    println!("Score: {}", score(428, 72061));
}

fn score(players: usize, rounds: usize) -> usize {
    let mut scores: Vec<usize> = vec![0; players];
    let mut player = 0;
    let mut game = Game::new();
    while game.next <= rounds {
        if let Some(score) = game.turn() {
            scores[player] += score;
        }
        player = (player + 1) % scores.len()
    }
    *scores.iter().max().unwrap()
}

#[derive(Debug)]
struct Game {
    marbles: Vec<usize>,
    position: usize,
    next: usize,
}

impl Game {
    pub fn new() -> Self {
        Game {
            marbles: vec![0],
            position: 1,
            next: 1,
        }
    }

    pub fn turn(&mut self) -> Option<usize> {
        let result = if self.next % 23 != 0 {
            if self.position < self.marbles.len() {
                self.marbles.insert(self.position + 1, self.next);
                self.position += 2
            } else {
                self.marbles.insert(1, self.next);
                self.position = 2;
            }
            None
        } else {
            if self.position > 7 {
                self.position -= 7;
            } else if self.marbles.len() > 7 {
                self.position = self.marbles.len() - 7 + self.position;
            } else {
                unimplemented!()
            }
            let score = self.next + self.marbles.remove(self.position - 1);
            Some(score)
        };
        self.next += 1;
        result
    }
}

