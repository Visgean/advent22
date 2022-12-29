use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashSet;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position {
    x: usize,
    y: usize,
    height: usize,
    cost: usize,
    value: usize,
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .value
            .cmp(&self.value)
            .then_with(|| self.value.cmp(&other.value))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn char_to_height(c: char) -> usize {
    if c == 'E' {
        return char_to_height('z');
    }
    if c == 'S' {
        return char_to_height('a');
    }
    c as usize
}

pub struct Map {
    map: Vec<Vec<usize>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

pub fn get_map() -> Map {
    let contents = fs::read_to_string("inputs/day12").expect("Where file");

    let mut map: Vec<Vec<usize>> = vec![];

    let mut start_x: usize = 0;
    let mut start_y: usize = 0;

    let mut end_x: usize = 0;
    let mut end_y: usize = 0;

    for (y, line) in contents.lines().enumerate() {
        match line.find('S') {
            None => {}
            Some(x) => {
                start_x = x;
                start_y = y
            }
        };
        match line.find('E') {
            None => {}
            Some(x) => {
                end_x = x;
                end_y = y
            }
        };

        map.push(line.chars().into_iter().map(char_to_height).collect());
    }

    Map {
        map,
        start_x,
        start_y,
        end_x,
        end_y,
    }
}

pub fn check_height(old: usize, new: usize) -> bool {
    new <= old || new == (old + 1)
}

pub fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    (x1 as isize - x2 as isize).abs() as usize + (y1 as isize - y2 as isize).abs() as usize
}

pub fn day12() -> Option<isize> {
    let map = get_map();
    let max_x = map.map[0].len() - 1;
    let max_y = map.map.len() - 1;

    let start = Position {
        x: map.start_x,
        y: map.start_y,
        height: map.map[0][0],
        cost: 0,
        value: 0,
    };

    let mut candidates: BinaryHeap<Position> = BinaryHeap::new();
    candidates.push(start);

    let mut visited: HashSet<(usize, usize, usize)> = HashSet::new();

    loop {
        let pos = candidates.pop().unwrap();

        if pos.x == map.end_x && pos.y == map.end_y {
            println!("Winning: {}", pos.cost);
            break;
        }

        if visited.contains(&(pos.x, pos.y, pos.cost)) {
            continue;
        };

        visited.insert((pos.x, pos.y, pos.cost));

        println!("trying: {}, {}, {}", pos.x, pos.y, pos.value);

        let mut next_moves: Vec<(usize, usize)> = vec![];
        if pos.x + 1 <= max_x {
            next_moves.push((pos.x + 1, pos.y))
        };
        if pos.y + 1 <= max_y {
            next_moves.push((pos.x, pos.y + 1))
        };

        if pos.x >= 1 {
            next_moves.push((pos.x - 1, pos.y))
        };
        if pos.y >= 1 {
            next_moves.push((pos.x, pos.y - 1))
        };

        for (x2, y2) in next_moves {
            let new_height = map.map[y2][x2];
            let new_cost = pos.cost + 1;
            if check_height(pos.height, new_height) && !visited.contains(&(x2, y2, new_cost)) {
                candidates.push(Position {
                    x: x2,
                    y: y2,
                    height: new_height,
                    cost: new_cost,
                    value: new_cost + manhattan_distance(x2, y2, map.end_x, map.end_y),
                });
            }
        }
    }

    Some(0)
}
