use std::collections::HashMap;
use std::fs;

fn main() {
    let input: String = fs::read_to_string("day14.txt").unwrap();

    let mut state = State {
        mask0: 0,
        mask1: 0,
        mem: HashMap::new(),
    };
    for stmt in input.lines() {
        step(&mut state, stmt);
    }
    println!("part 1: {}", state.mem.values().sum::<u64>());

    let mut state = StateV2 {
        mask: "".to_string(),
        mem: HashMap::new(),
    };
    let input: Vec<_> = input.lines().collect();
    for stmt in input.iter() {
        step_v2(&mut state, stmt);
    }
    println!("part 2: {}", state.mem.values().sum::<u64>());
}

struct State {
    mask0: u64,
    mask1: u64,
    mem: HashMap<u64, u64>,
}

fn step(state: &mut State, stmt: &str) {
    if stmt.starts_with("mask = ") {
        set_mask(state, stmt);
    } else if stmt.starts_with("mem") {
        set_mem(state, stmt);
    } else {
        panic!("bad statement: {}", stmt);
    }
}

fn set_mask(state: &mut State, stmt: &str) {
    let mask: String = stmt
        .chars()
        .filter(|c| ['0', '1', 'X'].contains(c))
        .collect();
    state.mask0 = mask
        .chars()
        .map(|c| char_to_bit(0, c))
        .fold(0, |a, b| 2 * a + b);
    state.mask1 = mask
        .chars()
        .map(|c| char_to_bit(1, c))
        .fold(0, |a, b| 2 * a + b);
}

fn char_to_bit(x: u64, c: char) -> u64 {
    match c {
        'X' => x,
        '1' => 1,
        '0' => 0,
        _ => panic!("expected X10: {}", c),
    }
}

fn set_mem(state: &mut State, stmt: &str) {
    let mut parts = stmt.split(" = ");
    let loc: u64 = parts
        .next()
        .unwrap()
        .strip_prefix("mem[")
        .unwrap()
        .strip_suffix("]")
        .unwrap()
        .parse()
        .expect(stmt);
    let value: u64 = parts.next().unwrap().parse().expect(stmt);
    let value = (value | state.mask0) & state.mask1;
    state.mem.insert(loc, value);
}

struct StateV2 {
    mask: String,
    mem: HashMap<u64, u64>,
}

fn step_v2(state: &mut StateV2, stmt: &str) {
    if stmt.starts_with("mask = ") {
        set_mask_v2(state, stmt);
    } else if stmt.starts_with("mem") {
        set_mem_v2(state, stmt);
    }
}

fn set_mask_v2(state: &mut StateV2, stmt: &str) {
    state.mask = stmt
        .chars()
        .filter(|c| ['0', '1', 'X'].contains(c))
        .collect();
}

fn set_mem_v2(state: &mut StateV2, stmt: &str) {
    // println!("{:?}", (&state.mask, stmt));
    let mut parts = stmt.split(" = ");
    let loc: u64 = parts
        .next()
        .unwrap()
        .strip_prefix("mem[")
        .unwrap()
        .strip_suffix("]")
        .unwrap()
        .parse()
        .expect(stmt);
    let value: u64 = parts.next().unwrap().parse().expect(stmt);
    for loc in apply_mask(loc, &state.mask) {
        state.mem.insert(loc, value);
    }
}

fn apply_mask(loc: u64, mask: &str) -> Vec<u64> {
    let mask1 = mask.replace("X", "0");
    let loc_ = loc | u64::from_str_radix(&mask1, 2).unwrap();
    let xs: Vec<u64> = mask
        .chars()
        .enumerate()
        .filter(|&(_i, c)| c == 'X')
        .map(|(i, _c)| i as u64)
        .collect();
    let loc__: u64 = loc_ & !(xs.iter().map(|b| 1 << (35 - b)).sum::<u64>());
    let mut result: Vec<u64> = Vec::new();
    for bits in subsequences(&xs[..]) {
        let v: u64 = bits.iter().map(|b| 1 << (35 - b)).sum();
        result.push(loc__ | v);
    }
    result
}

// fn subsequences<T>(xs: &[T]) -> Box<dyn Iterator<Item = dyn Iterator<Item = T>>> {
//     if xs.len() == 0 {
//         Box::new(iter::empty())
//     } else {
//         let x0: T = xs[0];
//         Box::new(
//             subsequences(&xs[1..])
//                 .map(|xs| iter::once(x0).chain(xs))
//                 .chain(subsequences(&xs[1..])),
//         )
//     }
// }

fn subsequences<T: Clone + Copy + std::fmt::Debug>(xs: &[T]) -> Vec<Vec<T>> {
    if xs.len() == 0 {
        return Vec::from([Vec::new()]);
    }

    let mut ss = subsequences(&xs[1..]);
    let mut ss_ = ss.clone();
    for s in ss_.iter_mut() {
        s.push(xs[0]);
    }
    ss.append(&mut ss_);
    ss
}
