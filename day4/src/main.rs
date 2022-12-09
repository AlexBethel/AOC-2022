use std::{fs::read_to_string, path::Path};

use chumsky::prelude::*;

#[derive(Debug, Clone)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    pub fn overlaps(&self, other: &Self) -> bool {
        if self.start > other.start {
            other.overlaps(self)
        } else {
            self.end >= other.start
        }
    }

    pub fn contains(&self, other: &Self) -> bool {
        (self.start <= other.start && self.end >= other.end)
            || (other.start <= self.start && other.end >= self.end)
    }
}

fn parse_range() -> impl Parser<char, Range, Error = Simple<char>> {
    let num = filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .at_least(1)
        .collect::<String>()
        .from_str()
        .unwrapped();

    num.then_ignore(just('-'))
        .then(num)
        .map(|(l, r)| Range { start: l, end: r })
}

#[derive(Debug, Clone)]
struct Line(Range, Range);

impl Line {
    pub fn has_overlap(&self) -> bool {
        self.0.overlaps(&self.1)
    }

    pub fn has_contain(&self) -> bool {
        self.0.contains(&self.1)
    }
}

fn parse_line() -> impl Parser<char, Line, Error = Simple<char>> {
    parse_range()
        .then_ignore(just(','))
        .then(parse_range())
        .map(|(l, r)| Line(l, r))
}

fn parse_file() -> impl Parser<char, Vec<Line>, Error = Simple<char>> {
    parse_line().separated_by(just('\n'))
}

fn main() {
    let input = read_to_string(Path::new("input")).unwrap();
    let lines = parse_file().parse(input).unwrap();
    println!(
        "No. containing = {}",
        lines
            .clone()
            .into_iter()
            .filter(|line| line.has_contain())
            .count()
    );
    println!(
        "No. overlap = {}",
        lines.into_iter().filter(|line| line.has_overlap()).count()
    );
}
