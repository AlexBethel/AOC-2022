fn find_header(input: &str, size: usize) -> usize {
    let (window_number, _window_contents) = input
        .as_bytes()
        .windows(size)
        .enumerate()
        .find(|(_num, window)| {
            window
                .iter()
                .enumerate()
                .all(|(idx, val)| !window[0..idx].contains(val))
        })
        .unwrap();
    window_number + size
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    println!("part 1: {}", find_header(&input, 4));
    println!("part 2: {}", find_header(&input, 14));
}
