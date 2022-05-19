use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone)]
enum Inst {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

#[derive(Debug)]
struct State {
    pc: usize,
    acc: i32,
    past_pc: HashSet<usize>,
}

fn main() {
    let ops: Vec<_> = fs::read_to_string("day8.txt")
        .expect("failed to load input")
        .lines()
        .map(parse_inst)
        .collect();

    let mut state = State {
        pc: 0,
        acc: 0,
        past_pc: HashSet::new(),
    };

    while !has_looped(&state) {
        state = step(&ops, state);
    }

    println!("part 1: {:?}", state.acc);

    for (i, _) in ops.iter().enumerate() {
        let ops = modify_prog(&ops, i);

        let mut state = State {
            pc: 0,
            acc: 0,
            past_pc: HashSet::new(),
        };

        while !has_looped(&state) && !has_terminated(&ops, &state) {
            state = step(&ops, state);
        }
        if has_terminated(&ops, &state) {
            println!("part 2: {:?}", state.acc);
            return;
        }
    }
}

fn parse_inst(line: &str) -> Inst {
    let mut words = line.split_whitespace();
    let (op, val) = (
        words.next().unwrap(),
        words.next().unwrap().parse().unwrap(),
    );
    match op {
        "acc" => Inst::Acc(val),
        "jmp" => Inst::Jmp(val),
        "nop" => Inst::Nop(val),
        _ => panic!("bad instruction"),
    }
}

fn step(prog: &[Inst], mut state: State) -> State {
    if has_terminated(prog, &state) {
        return state;
    }
    match prog[state.pc] {
        Inst::Acc(v) => {
            state.past_pc.insert(state.pc);
            state.pc += 1;
            state.acc += v;
        }
        Inst::Jmp(v) => {
            state.past_pc.insert(state.pc);
            state.pc = ((state.pc as i32) + v) as usize;
        }
        Inst::Nop(_v) => {
            state.past_pc.insert(state.pc);
            state.pc += 1;
        }
    }
    state
}

fn has_looped(state: &State) -> bool {
    state.past_pc.contains(&state.pc)
}

fn has_terminated(prog: &[Inst], state: &State) -> bool {
    state.pc >= prog.len()
}

fn modify_prog(prog: &[Inst], i: usize) -> Vec<Inst> {
    let mut modified: Vec<_> = prog.to_vec();

    match modified[i] {
        Inst::Jmp(v) => {
            modified[i] = Inst::Nop(v);
        }
        Inst::Nop(v) => {
            modified[i] = Inst::Jmp(v);
        }
        _ => {}
    }
    modified
}
