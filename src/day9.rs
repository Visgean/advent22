use std::collections::HashSet;
use std::fs;

struct Command {
    direction: char,
    repetition: usize,
}

impl Command {
    pub fn from_str(line: &str) -> Command {
        let mut split = line.split_whitespace();
        let direction = split.next().unwrap().chars().nth(0).unwrap();
        let repetition: usize = split.next().unwrap().parse().unwrap();

        Command {
            direction,
            repetition,
        }
    }

    pub fn get_vector(&self) -> (isize, isize) {
        if self.direction == 'U' {
            return (0, 1);
        }
        if self.direction == 'R' {
            return (1, 0);
        }
        if self.direction == 'L' {
            return (-1, 0);
        }
        if self.direction == 'D' {
            return (0, -1);
        }
        (0, 0)
    }
}

pub fn day9() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day9").expect("Where file");

    let mut rope: [(isize, isize); 10] = [(0, 0); 10];

    let mut tail_pos: HashSet<(isize, isize)> = HashSet::new();
    tail_pos.insert((0, 0));

    for line in contents.lines() {
        let command = Command::from_str(line);
        let (x2, y2) = command.get_vector();

        for _ in 0..command.repetition {
            rope[0].0 += x2;
            rope[0].1 += y2;

            for tail_index in 1..10 {
                let diff_x = rope[tail_index - 1].0 - rope[tail_index].0;
                let diff_y = rope[tail_index - 1].1 - rope[tail_index].1;

                let touching = (diff_x.abs() + diff_y.abs()) <= 1;
                let diagonal = diff_x.abs() == 1 && diff_y.abs() == 1;

                if !(touching || diagonal) {
                    if diff_x != 0 {
                        if rope[tail_index - 1].0 > rope[tail_index].0 {
                            rope[tail_index].0 += 1;
                        } else {
                            rope[tail_index].0 -= 1;
                        }
                    }
                    if diff_y != 0 {
                        if rope[tail_index - 1].1 > rope[tail_index].1 {
                            rope[tail_index].1 += 1;
                        } else {
                            rope[tail_index].1 -= 1;
                        }
                    }
                }
            }

            tail_pos.insert(rope[9]);
        }
    }

    println!("{}", tail_pos.len());
    Some(0)
}
