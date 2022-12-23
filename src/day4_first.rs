use std::fs;

pub struct PrimitiveRange {
    low: isize,
    high: isize,
}

impl PrimitiveRange {
    pub fn str_range(str_repr: &str) -> Option<PrimitiveRange> {
        let mut parts = str_repr.split('-');
        let low: isize = parts.next()?.parse().ok()?;
        let high: isize = parts.next()?.parse().ok()?;

        Some(PrimitiveRange { low, high })
    }

    pub fn contains(&self, other: &Self) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    pub fn partial_overlap(&self, other: &Self) -> bool {
        self.low <= other.low || self.high >= other.high
    }
}

pub fn day4() -> Option<isize> {
    let contents = fs::read_to_string("inputs/day4").expect("Where file");

    let mut score = 0;
    for line in contents.lines() {
        let mut ranges = line.split(',');
        let range1 = PrimitiveRange::str_range(ranges.next()?)?;
        let range2 = PrimitiveRange::str_range(ranges.next()?)?;

        if range1.contains(&range2) || range2.contains(&range1) {
            score += 1
        };
    }

    Some(score)
}
