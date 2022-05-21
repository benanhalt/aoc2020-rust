use std::collections::HashMap;
use std::fs;

type Grid = HashMap<(i32, i32), char>;

fn main() {
    let grid: Grid = fs::read_to_string("day11.txt")
        .unwrap()
        .lines()
        .enumerate()
        .flat_map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, p)| ((r as i32, c as i32), p))
        })
        .collect();

    part1(&grid);
    part2(&grid);
}

fn part1(grid0: &Grid) {
    let mut grid = grid0.clone();
    let mut cycles = 0;

    loop {
        let grid_: Grid = step1(&grid);
        cycles += 1;

        if grid_ == grid {
            break;
        }
        grid = grid_;
    }

    println!(
        "part 1: {} in {}",
        grid.values().filter(|p| **p == '#').count(),
        cycles
    );
}

fn step1(grid: &Grid) -> Grid {
    grid.iter()
        .map(|((r, c), p)| {
            let p_ = match p {
                'L' => {
                    if count_adjacent(&grid, *r, *c) == 0 {
                        '#'
                    } else {
                        *p
                    }
                }
                '#' => {
                    if count_adjacent(&grid, *r, *c) > 3 {
                        'L'
                    } else {
                        *p
                    }
                }
                _ => *p,
            };
            ((*r, *c), p_)
        })
        .collect()
}

fn count_adjacent(grid: &Grid, r: i32, c: i32) -> i32 {
    let mut count = 0;
    for dr in [-1, 0, 1].iter() {
        for dc in [-1, 0, 1].iter() {
            if (*dr, *dc) == (0, 0) {
                continue;
            }
            if grid.get(&(r + dr, c + dc)) == Some(&'#') {
                count += 1;
            }
        }
    }
    count
}

fn part2(grid0: &Grid) {
    let mut grid = grid0.clone();
    let mut cycles = 0;

    loop {
        let grid_: Grid = step2(&grid);
        cycles += 1;

        if grid_ == grid {
            break;
        }
        grid = grid_;
    }

    println!(
        "part 2: {} in {}",
        grid.values().filter(|p| **p == '#').count(),
        cycles
    );
}

fn step2(grid: &Grid) -> Grid {
    grid.iter()
        .map(|((r, c), p)| {
            let p_ = match p {
                'L' => {
                    if count_see(&grid, *r, *c) == 0 {
                        '#'
                    } else {
                        *p
                    }
                }
                '#' => {
                    if count_see(&grid, *r, *c) > 4 {
                        'L'
                    } else {
                        *p
                    }
                }
                _ => *p,
            };
            ((*r, *c), p_)
        })
        .collect()
}

fn count_see(grid: &Grid, r: i32, c: i32) -> i32 {
    let mut count = 0;

    for dr in [-1, 0, 1].iter() {
        for dc in [-1, 0, 1].iter() {
            if (*dr, *dc) == (0, 0) {
                continue;
            }
            let mut p: Option<&char> = None;
            for m in 1.. {
                p = grid.get(&(r + m * *dr, c + m * *dc));
                if p != Some(&'.') {
                    break;
                }
            }
            if p == Some(&'#') {
                count += 1;
            }
        }
    }
    count
}
