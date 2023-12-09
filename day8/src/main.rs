use rayon::prelude::*;
use std::{
    collections::HashMap,
    fs,
    iter::Cycle,
    process::exit,
    str::{Chars, Lines},
};

fn main() {
    let input = fs::read_to_string("../input/day8").unwrap();

    let mut lines = input.lines();
    let movement = lines.next().unwrap();

    lines.next().unwrap();
    let map = generate_map(lines);

    let num_moves = count_number_of_moves(movement.chars().cycle(), &map);
    println!("P1 {}", num_moves);
}

fn generate_map(lines: Lines) -> HashMap<&str, (&str, &str)> {
    lines
        .map(|line| {
            let (start, other_parts) = line.split_once(" = (").unwrap();
            (
                start,
                other_parts.trim_end_matches(')').split_once(", ").unwrap(),
            )
        })
        .collect()
}

fn count_number_of_moves(movement: Cycle<Chars>, map: &HashMap<&str, (&str, &str)>) -> u32 {
    let mut amount = 0;
    let mut start = "AAA";
    for move_dir in movement {
        amount += 1;
        let current_item = map.get(start).unwrap();
        start = match move_dir {
            'L' => current_item.0,
            'R' => current_item.1,
            _ => unreachable!(),
        };
        if start == "ZZZ" {
            return amount;
        }
    }
    amount
}
