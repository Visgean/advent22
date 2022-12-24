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

    let mut head = (0, 0);
    let mut tail = (0, 0);

    // let head_pos: Vec<(isize, isize)> = vec![];

    let mut tail_pos: HashSet<(isize, isize)> = HashSet::new();
    tail_pos.insert(tail);

    for line in contents.lines() {
        let command = Command::from_str(line);
        let (x2, y2) = command.get_vector();

        for _ in 0..command.repetition {
            head = (head.0 + x2, head.1 + y2);

            let diff_x = head.0 - tail.0;
            let diff_y = head.1 - tail.1;

            let touching = (diff_x.abs() + diff_y.abs()) <= 1;
            let diagonal = diff_x.abs() == 1 && diff_y.abs() == 1;

            println!("{:?}, {:?}, {}, {}", head, tail, touching, diagonal);

            if !(touching || diagonal) {
                println!("moving");
                if diff_x != 0 {
                    if head.0 > tail.0 {
                        tail.0 += 1;
                    } else {
                        tail.0 -= 1;
                    }
                }
                if diff_y != 0 {
                    if head.1 > tail.1 {
                        tail.1 += 1;
                    } else {
                        tail.1 -= 1;
                    }
                }

                println!("moved to: {:?}", tail);

                tail_pos.insert(tail);
            }
        }
    }

    println!("{}", tail_pos.len());
    Some(0)
}
