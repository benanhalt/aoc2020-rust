use std::fs;

fn main() {
    let contents = fs::read_to_string("day3.txt").expect("failed load input");
    let rows: Vec<_> = contents.lines().map(|line| line.as_bytes()).collect();

    // For part one we just count the number of trees encountered on
    // the path defined by the given steps down and to the right.
    println!("part 1: {}", trees(&rows, 1, 3));

    // Part two is the same except we count the number of trees on
    // several paths and multiply the results. Note the slopes here
    // are given as (right, down).
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "part 2: {}",
        slopes
            .iter()
            .map(|(right, down)| trees(&rows, *down, *right))
            .fold(1, |a, b| a * b)
    );
}

/// Count the number of trees "#" encountered on the grid given by
/// rows along the path with slope (down, right) subject to the wrap
/// around logic specified in the problem.
fn trees(rows: &[&[u8]], down: usize, right: usize) -> usize {
    rows.iter()
        .enumerate()
        .filter(|(i, _row)| i % down == 0) // Take each down'th row.
        // For each step down, take the cell right steps over from the
        // last.
        .map(|(i, row)| row[(right * i) % row.len()])
        .filter(|&cell| cell == b'#') // Keep the cells that are trees.
        .count()
}
