use std::fs;
use regex::Regex;
use lazy_static::lazy_static;

pub fn parse_matrix(lines: &Vec<&str>) -> Vec<Vec<char>> {
    // i could have created the matrix manually but lol lets actually parse it!
    // [F]         [L]     [M]
    // [T]     [H] [V] [G] [V]
    // [N]     [T] [D] [R] [N]     [D]
    // [Z]     [B] [C] [P] [B] [R] [Z]
    // [M]     [J] [N] [M] [F] [M] [V] [H]
    // [G] [J] [L] [J] [S] [C] [G] [M] [F]
    // [H] [W] [V] [P] [W] [H] [H] [N] [N]
    // [J] [V] [G] [B] [F] [G] [D] [H] [G]
    //  1   2   3   4   5   6   7   8   9

    let mut matrix: Vec<Vec<char>> = vec![];

    for (y, c) in lines[8].chars().enumerate() {
        let mut column: Vec<char> = vec![];

        if c == ' ' {
            continue;
        }

        for x in (0..8).rev() {
            match lines[x].chars().nth(y) {
                None => continue,
                Some(v) => {
                    if v != ' ' {
                        column.push(v)
                    }
                }
            };
        }
        matrix.push(column);
    }

    matrix
}


pub struct Command {
    source: usize,
    dest: usize,
    count: usize,
}

impl Command {
    pub fn from_str(src: &str) -> Option<Command> {
        lazy_static! {
            static ref COMMAND_PATTERN: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let parts = COMMAND_PATTERN.captures(src).unwrap();

        let count: usize = parts[1].parse().ok()?;
        let source: usize = parts[2].parse().ok()?;
        let dest: usize = parts[3].parse().ok()?;

        Some(Command {
            count: count,
            source: source -1,
            dest: dest -1,
        })

    }
}

pub fn day5() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day5").expect("Where file");
    let lines: Vec<&str> = contents.lines().collect();

    let mut matrix = parse_matrix(&lines);

    for line in &lines[10..] {
        let command = Command::from_str(&line)?;

        for _ in 0..command.count {
            let val = matrix[command.source].pop()?;
            matrix[command.dest].push(
                val
            )
        }
    }

    for column in matrix {
        print!("{}", column.last()?)
    }
    println!();

    Some(0)
}
