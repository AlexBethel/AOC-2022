use chumsky::prelude::*;

#[derive(Debug)]
struct StackRow(Vec<Option<char>>);

#[derive(Debug)]
struct StackRepr(Vec<StackRow>);

fn parse_stack_row() -> impl Parser<char, StackRow, Error = Simple<char>> {
    let filled = just('[')
        .ignore_then(any())
        .then_ignore(just(']'))
        .map(Some);

    let empty = just("   ").map(|_| None);

    filled.or(empty).separated_by(just(' ')).map(StackRow)
}

fn parse_stack_repr() -> impl Parser<char, StackRepr, Error = Simple<char>> {
    parse_stack_row().separated_by(just('\n')).map(StackRepr)
}

#[derive(Debug, Clone)]
struct Stacks(Vec<Vec<char>>);

impl From<StackRepr> for Stacks {
    fn from(repr: StackRepr) -> Self {
        let mut s = Vec::new();
        let width = repr
            .0
            .iter()
            .map(|row| row.0.len())
            .max()
            .expect("no stacks");

        for _ in 0..width {
            s.push(Vec::new());
        }

        for row in repr.0.into_iter().rev() {
            for (x, val) in row.0.into_iter().enumerate() {
                if let Some(v) = val {
                    s[x].push(v);
                }
            }
        }

        Self(s)
    }
}

fn parse_spacer_chunk() -> impl Parser<char, (), Error = Simple<char>> {
    filter(|c: &char| c.is_ascii_whitespace() || c.is_ascii_digit())
        .repeated()
        .map(|_| ())
}

#[derive(Debug)]
struct MoveLine {
    num: u32,
    from: u32,
    to: u32,
}

fn parse_move_line() -> impl Parser<char, MoveLine, Error = Simple<char>> {
    let num = filter(|c: &char| c.is_ascii_digit())
        .repeated()
        .collect::<String>()
        .from_str()
        .unwrapped();

    just("move ")
        .ignore_then(num)
        .then(just(" from ").ignore_then(num))
        .then(just(" to ").ignore_then(num))
        .map(|((num, from), to)| MoveLine { num, from, to })
}

#[derive(Debug)]
struct FileData {
    stacks: Stacks,
    moves: Vec<MoveLine>,
}

fn parse_file_data() -> impl Parser<char, FileData, Error = Simple<char>> {
    parse_stack_repr()
        .map(|repr| repr.into())
        .then_ignore(parse_spacer_chunk())
        .then(parse_move_line().separated_by(just('\n')))
        .map(|(stacks, moves)| FileData { stacks, moves })
}

impl MoveLine {
    pub fn apply(&self, is_part_2: bool, stacks: &mut Stacks) {
        if is_part_2 {
            let src_stack = &mut stacks.0[self.from as usize - 1];
            let elems = src_stack[(src_stack.len() - self.num as usize)..].to_owned();
            src_stack.truncate(src_stack.len() - self.num as usize);

            let dest_stack = &mut stacks.0[self.to as usize - 1];
            dest_stack.extend_from_slice(&elems);
        } else {
            for _ in 0..self.num {
                let v = stacks.0[self.from as usize - 1]
                    .pop()
                    .expect("popping from empty stack");
                stacks.0[self.to as usize - 1].push(v);
            }
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("input").unwrap();

    let FileData { stacks, moves } = parse_file_data().parse(file).expect("Format error");

    // Part 1
    let mut stacks_1 = stacks.clone();
    moves.iter().for_each(|m| m.apply(false, &mut stacks_1));
    let tops = stacks_1
        .0
        .into_iter()
        .map(|v| *v.last().unwrap())
        .collect::<String>();
    println!("Part 1: {}", tops);

    // Part 2
    let mut stacks_2 = stacks.clone();
    moves.into_iter().for_each(|m| m.apply(true, &mut stacks_2));
    let tops = stacks_2
        .0
        .into_iter()
        .map(|v| *v.last().unwrap())
        .collect::<String>();
    println!("Part 2: {}", tops);
}
