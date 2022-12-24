use std::collections::HashMap;
use std::fs;

pub fn day10() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day10").expect("Where file");

    let mut x_to_add: HashMap<isize, isize> = HashMap::new();

    let mut x = 1;
    let mut cycles = 1;
    let mut every20th = 0;

    for line in contents.lines() {
        if line == "noop" {
            cycles += 1;
        } else {
            let to_add: isize = line[5..].parse().unwrap();
            x_to_add.insert(cycles + 2, to_add);
            cycles += 2
        }
    }

    println!();

    for c in 1..cycles {
        if x_to_add.contains_key(&c) {
            x += x_to_add.get(&c)?;
            x_to_add.remove(&c);
        }

        let pos = (c - 1) % 40;
        if x == (pos - 1) || pos == x || (pos + 1) == x {
            print!("#");
        } else {
            print!(" ");
        }

        if c % 40 == 0 {
            println!();
        }
    }

    println!();
    println!("cycles: {cycles}");
    println!("Cycle strength: {every20th}");

    Some(0)
}
