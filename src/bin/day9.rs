use std::cmp;
use std::fs;

fn main() {
    let numbers: Vec<u64> = fs::read_to_string("day9.txt")
        .expect("failed to load input")
        .lines()
        .map(|line| line.parse().expect(line))
        .collect();

    let invalid = find_invalid(&numbers).unwrap();
    println!("part 1: {}", invalid);
    println!("part 2: {}", find_weakness(&numbers, invalid).unwrap());
}

fn find_invalid(numbers: &[u64]) -> Option<u64> {
    for i in 0..numbers.len() {
        let t = &numbers[i..];
        let (preamble, n) = (&t[..25], t[25]);
        if !all_sums(preamble).iter().any(|sum| *sum == n) {
            return Some(n);
        }
    }
    None
}

fn all_sums(numbers: &[u64]) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    for i in 0..(numbers.len() - 1) {
        let t = &numbers[i..];
        let a = t[0];
        for b in t[1..].iter() {
            result.push(a + b);
        }
    }
    result
}

fn find_weakness(numbers: &[u64], invalid: u64) -> Option<u64> {
    for i in 0..(numbers.len() - 1) {
        let t = &numbers[i..];
        let first = t[0];
        let rest = &t[1..];
        let mut sums = rest
            .iter()
            .scan((first, first, first), |(sum, min, max), x| {
                *sum = *sum + x;
                *min = cmp::min(*min, *x);
                *max = cmp::max(*max, *x);
                Some((*sum, *min, *max))
            })
            .skip(1) // Throw away the degenerate case where the sum is a single term.
            .skip_while(|(sum, _, _)| *sum < invalid); // Skip the sums that are too small.

        // Only need to check the first sum >= n since they are monotonically increasing.
        if let Some((sum, min, max)) = sums.next() {
            if sum == invalid {
                return Some(min + max);
            }
        }
    }
    None
}
