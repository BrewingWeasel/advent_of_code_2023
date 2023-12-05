use std::fs;

fn main() {
    let s = fs::read_to_string("../input/day4").unwrap();
    let lines: Vec<&str> = s.lines().collect();
    let new_stuff = lines.iter().map(|v| get_num_of_winners(v)).collect();
    println!("{}", new_run_lines(new_stuff));
}

fn new_run_lines(inp: Vec<usize>) -> usize {
    let mut times_called = vec![1; inp.len()];
    for (num_i, v) in inp.clone().iter().enumerate() {
        let copies_of_card = times_called[num_i];
        for times_called_i in times_called[num_i + 1..num_i + v + 1].iter_mut() {
            *times_called_i += 1 * copies_of_card;
        }
    }
    times_called.iter().sum::<usize>()
}

fn get_num_of_winners(inp: &str) -> usize {
    let all_values: Vec<Vec<&str>> = inp
        .split(" | ")
        .map(|v| v.split_whitespace().collect())
        .collect();
    let winners = &all_values[0];
    let attempts = &all_values[1];

    winners.iter().filter(|v| attempts.contains(v)).count()
}
