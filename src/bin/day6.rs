use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let input = fs::read_to_string("day6.txt").expect("failed to load input");
    let groups: Vec<_> = input.split("\n\n").collect();

    // just count up the unique characters in each group and sum over groups
    let part1: usize = groups
        .iter()
        .map(|group| {
            HashSet::<char>::from_iter(group.chars().filter(|c| c.is_ascii_alphabetic())).len()
        })
        .sum();
    println!("part 1: {}", part1);

    // for each group count the characters in common to all lines and
    // sum over all groups
    let part2: usize = groups
        .iter()
        .map(|group| {
            let answers: Vec<_> = group
                .split_whitespace()
                .map(|line| HashSet::<char>::from_iter(line.chars()))
                .collect();
            answers[0]
                .iter()
                .filter(|c| answers[1..].iter().all(|a| a.contains(c)))
                .count()
        })
        .sum();
    println!("part 2: {}", part2);
}
