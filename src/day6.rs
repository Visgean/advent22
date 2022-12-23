use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::fs;

pub fn day6() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day6").expect("Where file");

    let unique: usize = 14;
    let length = contents.len();
    let max_playable = length - unique;
    for x in 0..max_playable {
        let mut set = HashSet::new();

        for i in 0..unique {
            set.insert(contents.chars().nth(x + i)?);
        }

        if set.len() == unique {
            return Some((x + unique) as isize);
        }
    }

    Some(0)
}
