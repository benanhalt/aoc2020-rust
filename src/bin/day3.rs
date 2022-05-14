use std::fs;

fn main() {
    let contents = fs::read_to_string("day3.txt").expect("failed load input");
    let rows: Vec<_> = contents.lines().map(|line| line.as_bytes()).collect();

    println!("part 1: {}", trees(&rows, 1, 3));

    let slopes = vec![(1,1), (3,1), (5,1), (7, 1), (1, 2)];
    println!("part 2: {}",
             slopes.iter()
             .map(|(right, down)| trees(&rows, *down, *right))
             .fold(1, |a, b| a * b)
    );
}

fn trees(rows: &[&[u8]], down: usize, right: usize) -> usize {
    rows.iter().enumerate()
        .filter(|(i, _row)| i % down == 0)
        .map(|(i, row)| row[(right*i) % row.len()])
        .filter(|&cell| cell == b'#')
        .count()
}
