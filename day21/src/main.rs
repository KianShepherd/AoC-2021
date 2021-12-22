use std::fs;
use std::time::Instant;

#[derive(Debug)]
struct Die {
    last_roll: usize,
    max_roll: usize,
}

impl Die {
    fn roll(&mut self) -> usize {
        self.last_roll += 1;
        if self.last_roll > self.max_roll {
            self.last_roll = 1;
        }
        self.last_roll
    }
}

#[derive(Debug)]
struct GameState {
    p1_loc: usize,
    p2_loc: usize,
    p1_score: usize,
    p2_score: usize,
    rolls: usize,
    die: Die,
}
impl GameState {
    fn new(p1_start: usize, p2_start: usize) -> Self {
        Self {
            p1_loc: p1_start,
            p2_loc: p2_start,
            p1_score: 0,
            p2_score: 0,
            rolls: 0,
            die: Die {
                last_roll: 0,
                max_roll: 100,
            },
        }
    }

    fn play_game(&mut self) -> u32 {
        loop {
            let p1_roll = self.die.roll() + self.die.roll() + self.die.roll();
            self.p1_loc += p1_roll;
            while self.p1_loc > 10 {
                self.p1_loc -= 10;
            }
            self.p1_score += self.p1_loc;
            self.rolls += 3;
            if self.p1_score >= 1000 {
                println!("P1 W: {:?} - {:?}", self.p2_score, self.rolls);
                return self.p2_score as u32 * self.rolls as u32;
            }

            let p2_roll = self.die.roll() + self.die.roll() + self.die.roll();
            self.p2_loc += p2_roll;
            while self.p2_loc > 10 {
                self.p2_loc -= 10;
            }
            self.p2_score += self.p2_loc;
            self.rolls += 3;
            if self.p2_score >= 1000 {
                println!("{:?} - {:?}", self.p1_score, self.rolls);
                return self.p1_score as u32 * self.rolls as u32;
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let start = Instant::now();
    let mut v = contents.trim().split('\n');
    let p1 = v
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    let p2 = v
        .next()
        .unwrap()
        .split(' ')
        .last()
        .unwrap()
        .trim()
        .parse::<usize>()
        .unwrap();
    println!("{:?} : {:?}", p1, p2);
    let mut game = GameState::new(p1, p2);

    println!("{:?} : took {:?}", game.play_game(), start.elapsed());
}
