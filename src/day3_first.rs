use std::collections::HashSet;
use std::fs;

pub fn char_score(c: char) -> i32 {
    if !c.is_ascii_alphabetic() {
        return 0
    }

    if c.is_ascii_lowercase() {
        return c as i32 - (48 * 2)
    }

    return c as i32 - 38
}


pub fn day3() -> Option<()> {
    let contents = fs::read_to_string("inputs/day3").expect("Where file");

    let mut score = 0;
    for line in contents.lines() {
        let items: Vec<char> = line.chars().collect();
        let m = items.len() / 2;

        let first: HashSet<&char> = items[0..m].into_iter().collect();
        let second: HashSet<&char> = items[m..].into_iter().collect();

        let intersection = **first.intersection(&second).next()?;
        let char_score = char_score(intersection);

        score += char_score;


    }

    println!("{}", score);

    Some(())
}
