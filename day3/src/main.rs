use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    process::exit,
    str::FromStr,
};

fn char_to_priority(c: char) -> Result<u32, String> {
    if ('a'..='z').contains(&c) {
        Ok(c as u32 - 'a' as u32 + 1)
    } else if ('A'..='Z').contains(&c) {
        Ok(c as u32 - 'A' as u32 + 27)
    } else {
        Err(format!("invalid character `{c}`"))
    }
}

struct Rucksack {
    // Contents of the left and right compartments, as priority numbers.
    left: Vec<u32>,
    right: Vec<u32>,
}

impl FromStr for Rucksack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, r) = if s.len() % 2 == 0 {
            Ok(s.split_at(s.len() / 2))
        } else {
            Err(format!("Length of `{s}` is odd"))
        }?;

        let to_priorities = |slice: &str| {
            slice
                .chars()
                .map(|c| char_to_priority(c))
                .collect::<Result<_, _>>()
        };

        Ok(Self {
            left: to_priorities(l)?,
            right: to_priorities(r)?,
        })
    }
}

impl Rucksack {
    pub fn duplicate_priority(&self) -> Result<u32, String> {
        self.left
            .iter()
            .cloned()
            .find(|lv| self.right.contains(lv))
            .ok_or_else(|| "No duplicate found".to_string())
    }

    pub fn load_file(path: &Path) -> Result<Vec<Self>, String> {
        let file = BufReader::new(
            File::open(path).map_err(|e| format!("Error opening file {path:?}: {e}"))?,
        );

        file.lines()
            .map(|line| {
                line.map_err(|e| format!("Error reading line: {e}"))?
                    .parse()
            })
            .collect::<Result<_, _>>()
    }

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = &'a u32> {
        Iterator::chain(self.left.iter(), self.right.iter())
    }

    pub fn contains(&self, n: u32) -> bool {
        self.left.contains(&n) || self.right.contains(&n)
    }
}

fn total_priorities(path: &Path) -> Result<u32, String> {
    let rucksacks = Rucksack::load_file(path)?;
    rucksacks
        .into_iter()
        .map(|rucksack| rucksack.duplicate_priority())
        .try_fold(0, |l, res| Ok(l + res?))
}

fn part_1() {
    match total_priorities(Path::new("input")) {
        Ok(total) => println!("Total = {total}"),
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    }
}

struct ElfGroup(Vec<Rucksack>);

impl ElfGroup {
    pub fn group(rucksacks: Vec<Rucksack>, group_size: u32) -> Result<Vec<Self>, String> {
        let mut rucksacks = rucksacks.into_iter();
        let mut res = Vec::new();

        while let Some(next) = rucksacks.next() {
            let mut group = Vec::new();
            group.push(next);
            for _ in 0..(group_size - 1) {
                group.push(
                    rucksacks
                        .next()
                        .ok_or_else(|| "Wrong size group".to_string())?,
                );
            }

            res.push(ElfGroup(group));
        }

        Ok(res)
    }

    pub fn badge(&self) -> Result<u32, String> {
        self.0[0]
            .iter()
            .cloned()
            .find(|&n| self.0[1].contains(n) && self.0[2].contains(n))
            .ok_or_else(|| "No badge found".to_string())
    }
}

fn total_badges(path: &Path) -> Result<u32, String> {
    let rucksacks = Rucksack::load_file(path)?;
    let groups = ElfGroup::group(rucksacks, 3)?;
    groups
        .into_iter()
        .map(|group| group.badge())
        .try_fold(0, |l, res| Ok(l + res?))
}

fn part_2() {
    match total_badges(Path::new("input")) {
        Ok(total) => println!("Badge total = {total}"),
        Err(e) => {
            println!("Error: {e}");
            exit(1);
        }
    }
}

fn main() {
    part_1();
    part_2();
}
