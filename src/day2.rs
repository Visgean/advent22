use crate::day2::GameResult::{Draw, Lose, Win};
use crate::day2::Move::{Paper, Rock, Scissors};
use std::fs;

// X for Rock, Y for Paper, and Z for Scissors

pub enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    pub fn from_char(c: char) -> Option<GameResult> {
        match c {
            'X' => Some(Lose),
            'Y' => Some(Draw),
            'Z' => Some(Win),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn from_char(c: char) -> Option<Move> {
        match c {
            'A' => Some(Rock),
            'B' => Some(Paper),
            'C' => Some(Scissors),
            _ => None,
        }
    }

    pub fn score(&self) -> isize {
        // 1 for Rock, 2 for Paper, and 3 for Scissors
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
}

pub fn game_score(opp: Move, my: Move) -> isize {
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).
    match opp {
        Rock => match my {
            Rock => 3,
            Paper => 6,
            Scissors => 0,
        },
        Paper => match my {
            Rock => 0,
            Paper => 3,
            Scissors => 6,
        },
        Scissors => match my {
            Rock => 6,
            Paper => 0,
            Scissors => 3,
        },
    }
}

pub fn get_move(opp: Move, target: GameResult) -> Move {
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).
    match opp {
        Rock => match target {
            Win => Paper,
            Lose => Scissors,
            Draw => Rock,
        },
        Paper => match target {
            Win => Scissors,
            Lose => Rock,
            Draw => Paper,
        },
        Scissors => match target {
            Win => Rock,
            Lose => Paper,
            Draw => Scissors,
        },
    }
}

pub fn day2() -> Option<()> {
    let contents = fs::read_to_string("inputs/day2").expect("cant read day 2");

    let mut opp_move: Move;
    let mut my_move: Move;
    let mut target_outcome: GameResult;

    let mut tot_score = 0;

    for line in contents.lines() {
        opp_move = Move::from_char(line.chars().nth(0)?)?;
        target_outcome = GameResult::from_char(line.chars().nth(2)?)?;
        my_move = get_move(opp_move.clone(), target_outcome);

        // my_move = Move::from_char(line.chars().nth(2)?)?;

        tot_score += my_move.score() + game_score(opp_move, my_move);
    }

    println!("{}", tot_score);

    Some(())
}
