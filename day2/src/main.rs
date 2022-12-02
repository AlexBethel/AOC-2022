use std::cmp::Ordering;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("Score 1 = {}", score_1(&input));
    println!("Score 2 = {}", score_2(&input));
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Move {
    pub fn new(n: u32) -> Self {
        match n {
            0 => Move::Rock,
            1 => Move::Paper,
            2 => Move::Scissors,
            _ => panic!("{}", n),
        }
    }

    pub fn beater(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    pub fn loser(self) -> Self {
        // I'm lazy lol
        self.beater().beater()
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(Ord::cmp(self, other))
    }
}

impl Ord for Move {
    fn cmp(&self, other: &Self) -> Ordering {
        use {Move::*, Ordering::*};
        match (self, other) {
            (Rock, Rock) => Equal,
            (Rock, Paper) => Less,
            (Rock, Scissors) => Greater,
            (Paper, Rock) => Greater,
            (Paper, Paper) => Equal,
            (Paper, Scissors) => Less,
            (Scissors, Rock) => Less,
            (Scissors, Paper) => Greater,
            (Scissors, Scissors) => Equal,
        }
    }
}

fn total_scores(inputs: impl Iterator<Item = (Move, Move)>) -> u32 {
    inputs
        .map(|(opp, you)| {
            you as u32
                + match Ord::cmp(&you, &opp) {
                    Ordering::Less => 0,
                    Ordering::Equal => 3,
                    Ordering::Greater => 6,
                }
        })
        .sum()
}

fn score_1(input: &str) -> u32 {
    let moves = input.lines().map(|line| {
        let (opp_n, you_n) = parse_line(line);
        (Move::new(opp_n), Move::new(you_n))
    });

    total_scores(moves)
}

fn score_2(input: &str) -> u32 {
    let moves = input.lines().map(|line| {
        let (opp_n, you_n) = parse_line(line);
        let opp = Move::new(opp_n);
        (
            opp,
            match you_n {
                0 => opp.loser(),
                1 => opp,
                2 => opp.beater(),
                _ => unreachable!()
            },
        )
    });

    total_scores(moves)
}

fn parse_line(line: &str) -> (u32, u32) {
    let mut segments = line.split(' ');
    let first = segments.next().unwrap();
    let second = segments.next().unwrap();

    (
        match first {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("{}", first),
        },
        match second {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => panic!("{}", second),
        },
    )
}
