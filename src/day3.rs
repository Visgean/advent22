use std::collections::HashSet;
use std::fs;

pub fn char_score(c: char) -> i32 {
    if !c.is_ascii_alphabetic() {
        return 0;
    }

    if c.is_ascii_lowercase() {
        return c as i32 - (48 * 2);
    }

    return c as i32 - 38;
}

pub fn day3() -> Option<()> {
    let contents = fs::read_to_string("inputs/day3").expect("Where file");

    let mut score = 0;

    let lines: Vec<&str> = contents.lines().collect();

    for line in lines.chunks(3) {
        let a = line[0];
        let b = line[1];
        let c = line[2];

        let first: HashSet<char> = a.chars().collect();
        let second: HashSet<char> = b.chars().collect();
        let third: HashSet<char> = c.chars().collect();

        let intersection_ab: HashSet<char> = first.intersection(&second).cloned().collect();
        let intersection = *third.intersection(&intersection_ab).next()?;

        let char_score = char_score(intersection);

        score += char_score;
    }

    println!("{}", score);

    Some(())
}
