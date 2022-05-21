use std::fs;

fn main() {
    let input: String = fs::read_to_string("day13.txt").unwrap();
    let depart: i64 = input.lines().nth(0).unwrap().parse().unwrap();
    let buses: Vec<_> = input.lines().nth(1).unwrap().split(",").collect();

    part1(depart, &buses);
    part2(&buses);
}

fn part1(depart: i64, buses: &[&str]) {
    let buses: Vec<i64> = buses
        .iter()
        .filter(|&&b| b != "x")
        .map(|t| t.parse().unwrap())
        .collect();

    let mut wait_times: Vec<(i64, i64)> = buses.iter().map(|&b| (b, b - depart % b)).collect();
    wait_times.sort_by_key(|&(_t, b)| b);
    println!("part 1: {:?}", wait_times[0].0 * wait_times[0].1);
}

fn part2(buses: &[&str]) {
    let pairs: Vec<_> = buses
        .iter()
        .enumerate()
        .filter(|&(_i, &b)| b != "x")
        .map(|(d, b_)| {
            let b: i64 = b_.parse().unwrap();
            (b - d as i64, b)
        })
        .collect();
    println!("part 2: {:?}", chinese_remainder(&pairs).0);
}

fn inverse_(t: i64, t_: i64, r: i64, r_: i64) -> i64 {
    if r_ == 0 {
        if r > 1 {
            panic!("not invertible")
        } else {
            t
        }
    } else {
        let q = r / r_;
        inverse_(t_, t - q * t_, r_, r - q * r_)
    }
}

fn inverse(a: i64, n: i64) -> i64 {
    inverse_(0, 1, n, a)
}

fn chinese_remainder(pairs: &[(i64, i64)]) -> (i64, i64) {
    let big_n = pairs.iter().map(|(_a, b)| b).product();
    let x: i64 = pairs
        .iter()
        .map(|&(b, n)| {
            let n_ = big_n / n;
            let x = inverse(n_, n);
            b * n_ * x
        })
        .sum();
    (x % big_n, big_n)
}
