use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("../input/day1_1").unwrap();

    let possible_digits = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut total = 0;
    for line in contents.lines() {
        let mut start = None;
        let mut last = None;
        let mut so_far = String::new();
        for c in line.chars() {
            if c.is_ascii_digit() {
                last = Some(c.to_digit(10).unwrap());
                if start.is_none() {
                    start = last;
                }
            }
            so_far.push(c);
            for (ending, value) in &possible_digits {
                if so_far.ends_with(ending) {
                    last = Some(*value);
                    if start.is_none() {
                        start = last;
                    }
                }
            }
        }
        let single =
            start.expect("at least 1 number") * 10 + last.expect("should have already failed");
        total += single;
    }
    println!("{total}")
}
