use std::fs;

fn main() {
    let input = fs::read_to_string("../input/day6").unwrap();
    let (time, distance) = input.split_once('\n').unwrap();

    let amount: usize = get_all_nums(time)
        .iter()
        .zip(get_all_nums(distance).iter())
        .map(|(time, distance)| get_all_winning(*time, *distance))
        .product();
    println!("p1 {}", amount);

    let amount = get_all_winning(get_all_nums_part2(time), get_all_nums_part2(distance));
    println!("p2 {}", amount);
}

fn get_all_nums(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .skip(1)
        .map(|v| v.parse().unwrap())
        .collect()
}

fn get_all_nums_part2(input: &str) -> i64 {
    input
        .split_whitespace()
        .skip(1)
        .flat_map(|n| n.chars())
        .map(|v| v.to_digit(10).unwrap() as i64)
        .fold(0, |acc, x| acc * 10 + x)
}

fn get_all_winning(time: i64, distance: i64) -> usize {
    let mut amount_won = 0;
    let mut has_won = false;
    for time_held_down in 0..distance {
        if time_held_down * (time - time_held_down) > distance {
            amount_won += 1;
            has_won = true;
        } else if has_won {
            return amount_won;
        }
    }
    amount_won
}
