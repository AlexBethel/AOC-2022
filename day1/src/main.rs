fn prob1() -> usize {
    let text = std::fs::read_to_string("counts.txt").unwrap();
    let elves = text.split("\n\n");
    let fruits = elves.map(|elf| {
        elf.split('\n')
            .map(|fruit| fruit.parse::<usize>().unwrap_or(0))
    });

    fruits.map(Iterator::sum).max().unwrap()
}

fn prob2() -> usize {
    let text = std::fs::read_to_string("counts.txt").unwrap();
    let elves = text.split("\n\n");
    let fruits = elves.map(|elf| {
        elf.split('\n')
            .map(|fruit| fruit.parse::<usize>().unwrap_or(0))
    });
    let mut elf_sums: Vec<usize> = fruits.map(|elf| elf.sum()).collect();

    elf_sums.sort();
    elf_sums[elf_sums.len() - 3..].iter().sum()
}

fn main() {
    println!("1. Maximum calories = {}", prob1());
    println!("2. Top 3 = {}", prob2());
}
