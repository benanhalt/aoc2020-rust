use std::collections::HashMap;
use std::fs;

fn main() {
    let adapters: Vec<i32> = fs::read_to_string("day10.txt")
        .expect("failed to load input")
        .lines()
        .map(|line| line.parse().expect(line))
        .collect();

    let mut adapters_ = adapters.clone();
    adapters_.push(0);
    adapters_.sort();
    let mut diffs: Vec<_> = adapters_
        .iter()
        .zip(adapters_.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect();
    diffs.push(3);
    let ones = diffs.iter().filter(|d| **d == 1).count();
    let threes = diffs.iter().filter(|d| **d == 3).count();
    println!("part 1: {}", ones * threes);

    println!("part 2: {}", count_ways(&adapters, 0));
}

fn count_ways(available: &[i32], prev: i32) -> u64 {
    let goal = available.iter().max().unwrap();
    let mut memo: HashMap<i32, u64> = HashMap::new();

    fn count_(available: &[i32], memo: &mut HashMap<i32, u64>, goal: i32, prev: i32) -> u64 {
        match memo.get(&prev) {
            Some(count) => *count,
            None => {
                let count = if prev == goal {
                    1
                } else {
                    can_connect(available, prev)
                        .iter()
                        .map(|a| count_(available, memo, goal, *a))
                        .sum()
                };
                memo.insert(prev, count);
                count
            }
        }
    }

    count_(available, &mut memo, *goal, prev)
}

fn can_connect(available: &[i32], prev: i32) -> Vec<i32> {
    available
        .iter()
        .copied()
        .filter(|a| [1, 2, 3].contains(&(*a - prev)))
        .collect()
}
