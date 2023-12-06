use std::fs;

fn main() {
    let input = fs::read_to_string("../input/day5").unwrap();

    let mut sections = input.split("\n\n");
    let seeds = get_seeds(&sections.next().unwrap());

    let maps: Vec<AlmanacMap> = sections.map(|v| get_map(v)).collect();

    let min = get_min_seeds(&seeds, &maps);

    // let (min, error_marg) = seeds
    //     .iter()
    //     .map(|(seed, error_marg)| {
    //         let mut current_item = "seed";
    //         let mut new_seed = *seed;
    //         for map in &maps {
    //             if map.start == current_item {
    //                 current_item = map.end;
    //                 new_seed = map.apply_map(new_seed);
    //             }
    //         }
    //         (new_seed, error_marg)
    //     })
    //     .min()
    //     .unwrap();
    //
    // if *error_marg > 1 {
    //     println!("{min} from {} to {}", min - error_marg, min + error_marg)
    // }

    println!("MINIMUM: {}", min);
}

fn get_min_seeds(seeds: &[(usize, usize)], maps: &[AlmanacMap]) -> usize {
    let (min, old, error_marg) = seeds
        .iter()
        .map(|(seed, error_marg)| {
            let mut current_item = "seed";
            let mut new_seed = *seed;
            for map in maps {
                if map.start == current_item {
                    current_item = map.end;
                    new_seed = map.apply_map(new_seed);
                }
            }
            (new_seed, seed, error_marg)
        })
        .reduce(
            |(min_acc, acc_old, acc_error_marg), (min, new_old, error_marg)| {
                if min < min_acc {
                    (min, new_old, error_marg)
                } else {
                    (min_acc, acc_old, acc_error_marg)
                }
            },
        )
        .unwrap();
    if *error_marg < 1 {
        min
    } else {
        get_min_seeds(&generate_ranges(old - *error_marg, *error_marg * 2), maps)
    }
}

fn generate_ranges(num: usize, len: usize) -> Vec<(usize, usize)> {
    let mut nums = Vec::new();
    for i in 1..100 {
        nums.push((num + len / i, len / 10));
    }
    nums
}

fn get_seeds(input: &str) -> Vec<(usize, usize)> {
    let mut input_nums = input
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .map(|v| v.parse().unwrap());
    let mut nums = Vec::new();
    while let Some(num) = input_nums.next() {
        let mut individ_nums = generate_ranges(num, input_nums.next().unwrap());
        nums.append(&mut individ_nums);
    }
    nums
}

fn get_map(input: &str) -> AlmanacMap {
    let mut lines = input.lines();
    let (start, end) = lines
        .next()
        .unwrap()
        .trim_end_matches(" map:")
        .split_once("-to-")
        .unwrap();

    AlmanacMap {
        start,
        end,
        maps: lines.map(|v| get_individual_map(v)).collect(),
    }
}

struct AlmanacMap<'a> {
    start: &'a str,
    end: &'a str,
    maps: Vec<IndividualMap>,
}

impl AlmanacMap<'_> {
    fn apply_map(&self, input: usize) -> usize {
        for map in &self.maps {
            if input >= map.source_range && input < map.source_range + map.range_len {
                return input - map.source_range + map.destination_range;
            }
        }
        return input;
    }
}

struct IndividualMap {
    destination_range: usize,
    source_range: usize,
    range_len: usize,
}

fn get_individual_map(line: &str) -> IndividualMap {
    let mut parts = line.split_whitespace().map(|v| v.parse().unwrap());
    IndividualMap {
        destination_range: parts.next().unwrap(),
        source_range: parts.next().unwrap(),
        range_len: parts.next().unwrap(),
    }
}
