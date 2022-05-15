use std::fs;

fn main() {
    let input = fs::read_to_string("day5.txt").expect("failed to load input");
    let mut passes: Vec<_> = input
        .split_whitespace()
        .map(|seat_id| {
            seat_id
                .chars()
                .map(|c| match c {
                    'F' => 0,
                    'B' => 1,
                    'R' => 1,
                    'L' => 0,
                    _ => panic!("invalid character in seat id"),
                })
                .fold(0, |a, b| 2 * a + b)
        })
        .collect();
    passes.sort();

    println!("part 1: {}", passes.last().unwrap());

    println!(
        "part 2: {}",
        1 + passes
            .iter()
            .zip(&passes[1..])
            .take_while(|(&s, &t)| s + 1 == t)
            .last()
            .unwrap()
            .1
    );
}
