use regex::Regex;
use std::fs;

/// Represents a password policy line from the input
/// of the form "i-j c: passwd"
struct Policy {
    i: usize,
    j: usize,
    c: u8,
    passwd: String,
}

fn main() {
    let policy_re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();

    let parse = |line| {
        let captures = policy_re
            .captures(line)
            .expect(&format!("{} should match the regex", line));
        Policy {
            i: captures.get(1).unwrap().as_str().parse().unwrap(),
            j: captures.get(2).unwrap().as_str().parse().unwrap(),
            c: captures.get(3).unwrap().as_str().as_bytes()[0],
            passwd: captures.get(4).unwrap().as_str().to_string(),
        }
    };

    let contents = fs::read_to_string("day2.txt").expect("failed to load input");
    let policies: Vec<Policy> = contents.lines().map(parse).collect();
    println!("part 1: {}", part1(&policies));
    println!("part 2: {}", part2(&policies));
}

/// Count the number of policies that are valid according to the rule
/// that the number of characters matching c is between i and j.
fn part1(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let count = policy
                .passwd
                .as_bytes()
                .iter()
                .filter(|&&c| c == policy.c)
                .count();
            policy.i <= count && count <= policy.j
        })
        .count()
}

/// Count the number of policies that are valid according to the rule
/// the exactly one of the characters at i and j be equal to c. NB
/// that i and j are one-based indices.
fn part2(policies: &[Policy]) -> usize {
    policies
        .iter()
        .filter(|policy| {
            let bytes = policy.passwd.as_bytes();
            (bytes[policy.i - 1] == policy.c) ^ (bytes[policy.j - 1] == policy.c)
        })
        .count()
}
