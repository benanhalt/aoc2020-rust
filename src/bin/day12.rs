use std::fs;

fn main() {
    let input: String = fs::read_to_string("day12.txt").unwrap();
    let lines: Vec<_> = input.lines().collect();

    part1(&lines);
    part2(&lines);
}

struct State {
    heading: i32,
    x: i32,
    y: i32,
}

fn part1(input: &[&str]) {
    let mut state = State {
        heading: 90,
        x: 0,
        y: 0,
    };
    for inst in input.iter() {
        state = step(inst, &state);
    }
    println!("part 1: {}", state.x.abs() + state.y.abs());
}

fn mod360(h: i32) -> i32 {
    ((h % 360) + 360) % 360
}

fn step(inst: &str, state: &State) -> State {
    let amount = inst[1..].parse().unwrap();
    let dir = inst.chars().next().unwrap();
    match dir {
        'F' => forward(heading_to_dir(state.heading), amount, state),
        'L' => State {
            heading: mod360(state.heading - amount),
            ..*state
        },
        'R' => State {
            heading: mod360(state.heading + amount),
            ..*state
        },
        d => forward(d, amount, state),
    }
}

fn forward(dir: char, amount: i32, state: &State) -> State {
    match dir {
        'N' => State {
            y: state.y + amount,
            ..*state
        },
        'S' => State {
            y: state.y - amount,
            ..*state
        },
        'E' => State {
            x: state.x + amount,
            ..*state
        },
        'W' => State {
            x: state.x - amount,
            ..*state
        },
        d => panic!("only compass points: {}", d),
    }
}

fn heading_to_dir(h: i32) -> char {
    match h {
        0 => 'N',
        90 => 'E',
        180 => 'S',
        270 => 'W',
        h => panic!("only compass points: {}", h),
    }
}

#[derive(Clone)]
struct State2 {
    x: i32,
    y: i32,
    wpx: i32,
    wpy: i32,
}

fn part2(input: &[&str]) {
    let mut state = State2 {
        x: 0,
        y: 0,
        wpx: 10,
        wpy: 1,
    };
    for inst in input.iter() {
        state = step2(inst, &state);
    }
    println!("part 2: {}", state.x.abs() + state.y.abs());
}

fn step2(inst: &str, state: &State2) -> State2 {
    let amount: i32 = inst[1..].parse().unwrap();
    let dir = inst.chars().next().unwrap();
    match dir {
        'N' => State2 {
            wpy: state.wpy + amount,
            ..*state
        },
        'S' => State2 {
            wpy: state.wpy - amount,
            ..*state
        },
        'E' => State2 {
            wpx: state.wpx + amount,
            ..*state
        },
        'W' => State2 {
            wpx: state.wpx - amount,
            ..*state
        },
        'F' => State2 {
            x: state.x + amount * state.wpx,
            y: state.y + amount * state.wpy,
            ..*state
        },
        'L' => {
            let mut state_ = state.clone();
            for _i in 0..amount / 90 {
                state_ = State2 {
                    wpx: -state_.wpy,
                    wpy: state_.wpx,
                    ..state_
                };
            }
            state_
        }
        'R' => {
            let mut state_ = state.clone();
            for _i in 0..amount / 90 {
                state_ = State2 {
                    wpx: state_.wpy,
                    wpy: -state_.wpx,
                    ..state_
                };
            }
            state_
        }
        d => panic!("only compass points: {}", d),
    }
}
