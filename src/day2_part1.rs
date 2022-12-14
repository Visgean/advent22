use std::fs;
use crate::day2::Move::{Paper, Rock, Scissors};


// X for Rock, Y for Paper, and Z for Scissors

pub enum Move {
    Rock,
    Paper,
    Scissors
}




impl Move {
    pub fn from_char (c: char) -> Option<Move> {
        match c {
            'X' => Some(Rock),
            'Y' => Some(Paper),
            'Z' => Some(Scissors),

            'A' => Some(Rock),
            'B' => Some(Paper),
            'C' => Some(Scissors),
            _ => None
        }
    }

    pub fn score(&self) -> isize {
        // 1 for Rock, 2 for Paper, and 3 for Scissors
        match self {
            Rock => {1}
            Paper => {2}
            Scissors => {3}
        }
    }
}


pub fn game_score(opp: Move, my: Move) -> isize {
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).
    match opp {
        Rock => {
            match my {
                Rock => {3}
                Paper => {6}
                Scissors => {0}
            }
        }
        Paper => {
            match my {
                Rock => {0}
                Paper => {3}
                Scissors => {6}
            }
        }
        Scissors => {
            match my {
                Rock => {6}
                Paper => {0}
                Scissors => {3}
            }
        }
    }
}



pub fn day2() -> Option<()>{
    let contents = fs::read_to_string("inputs/day2").expect("cant read day 2");

    let mut opp_move:  Move;
    let mut my_move: Move;
    let mut tot_score = 0;

    for line in contents.lines(){
        opp_move = Move::from_char(line.chars().nth(0)?)?;
        my_move = Move::from_char(line.chars().nth(2)?)?;

        tot_score += my_move.score() + game_score(opp_move, my_move);
    }

    println!("{}", tot_score);

    Some(())



}