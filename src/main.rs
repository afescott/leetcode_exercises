use std::io::{self, BufRead};

/*
 * Complete the 'minimumBribes' function below.
 *
 * The function accepts INTEGER_ARRAY q as parameter.
 */

#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub score: i32,
}

pub struct Checker {
    players: Vec<Player>,
}

impl Checker {
    pub fn new(players: Vec<Player>) -> Self {
        Checker { players }
    }

    pub fn check(&mut self) {
        self.players.sort_by(|a, b| {
            println!("a: {:?}, b:{:?}", a, b);
            if a.score == b.score &&  {

            }
            b.score.cmp(&a.score)
        });

        for player in &self.players {
            println!("{} {}", player.name, player.score);
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let t = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut players: Vec<Player> = Vec::new();
    for _ in 0..t {
        let line = stdin_iterator.next().unwrap().unwrap();
        let mut parts = line.trim().split_whitespace();
        let name = parts.next().unwrap().to_string();
        let score = parts.next().unwrap().parse::<i32>().unwrap();

        players.push(Player { name, score });
    }

    let mut checker = Checker::new(players);
    checker.check();
}
