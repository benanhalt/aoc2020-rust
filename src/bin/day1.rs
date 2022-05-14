use std::fs;
use std::cmp::Ordering;

fn main() {
    let contents = fs::read_to_string("day1.txt").expect("failed to load input");
    let mut values: Vec<i32> = contents.lines().map(|x| x.parse().unwrap()).collect();
    values.sort();

    println!("part 1: {}", part1(&values));
    println!("part 2: {}", part2(&values));
}

/// Find a and b in values such that a + b = 2020 and return a * b.
fn part1(values: &Vec<i32>) -> i32 {
    for (i, a) in values.iter().enumerate() {
        let b = 2020 - a;

        // Since the values are sorted we can use binary search
        // to determine if b exists in the remainder of values.
        if binary_search(&values[i+1..], b) {
            return b * a;
        }
    }
    panic!("no solution");
}

/// Find a, b, c in values s.t. a + b + c = 2020 and return a * b * c.
fn part2(values: &Vec<i32>) -> i32 {
    for (i, a) in values.iter().enumerate() {
        let tail = &values[i+1..];

        for (j, b) in tail.iter().enumerate() {
            let c = 2020 - a - b;
            if c < 1 { break; }

            if binary_search(&tail[j+1..], c) {
                return c * b * a;
            }
        }
    }
    panic!("no solution");
}

fn binary_search(values: &[i32], target: i32) -> bool {
    let mut low = 0;
    let mut high = values.len() - 1;

    while high > low + 1 {
        let i = (high + low) / 2;

//        println!("step");
        match values[i].cmp(&target) {
            Ordering::Equal => {
                return true;
            }

            Ordering::Less => {
                low = i;
            }

            Ordering::Greater => {
                high = i;
            }
        }
    }

    false
}
