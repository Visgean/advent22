use std::collections::HashSet;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;


pub fn day6() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day6").expect("Where file");

    let length = contents.len();
    let max_playable = length - 4;
    for x in 0..max_playable {
        let mut set = HashSet::new();

        set.insert(contents.chars().nth(x)?);
        set.insert(contents.chars().nth(x + 1)?);
        set.insert(contents.chars().nth(x + 2)?);
        set.insert(contents.chars().nth(x + 3)?);

        if set.len() == 4 {
            return Some(x as isize + 4)
        }
    }


    Some(0)
}
