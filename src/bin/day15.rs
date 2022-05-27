use std::collections::HashMap;

fn main() {
    let input: [i32; 6] = [17, 1, 3, 16, 19, 0];
    let map: HashMap<i32, i32> = input[0..input.len() - 1]
        .iter()
        .copied()
        .enumerate()
        .map(|(i, v)| (v, i as i32))
        .collect();

    println!(
        "part 1: {}",
        game(
            2020,
            input.len() as i32,
            &mut map.clone(),
            input[input.len() - 1]
        )
    );
    println!(
        "part 2: {}",
        game(
            30000000,
            input.len() as i32,
            &mut map.clone(),
            input[input.len() - 1]
        )
    );
}

fn game(end: i32, mut count: i32, map: &mut HashMap<i32, i32>, mut prev: i32) -> i32 {
    while count != end {
        let next = match map.get(&prev) {
            Some(j) => count - j - 1,
            None => 0,
        };
        map.insert(prev, count - 1);
        count += 1;
        prev = next;
    }
    prev
}
