use std::fs;

fn main() {
    let input = fs::read_to_string("../input/day5").unwrap();

    let mut sections = input.split("\n\n");
    let seeds = get_seeds(&sections.next().unwrap());

    let maps: Vec<AlmanacMap> = sections.map(|v| get_map(v)).collect();

    let min = seeds
        .iter()
        .map(|seed| {
            let mut current_item = "seed";
            let mut new_seed = *seed;
            for map in &maps {
                if map.start == current_item {
                    current_item = map.end;
                    new_seed = map.apply_map(new_seed);
                }
            }
            new_seed
        })
        .min()
        .unwrap();

    println!("{}", min);
}

fn get_seeds(input: &str) -> Vec<usize> {
    let nums = input.split(": ").nth(1).unwrap();
    nums.split(" ").map(|v| v.parse().unwrap()).collect()
}

// fn get_seeds_part2(input: &str) -> Vec<usize> {
//     let mut input_nums = input
//         .split(": ")
//         .nth(1)
//         .unwrap()
//         .split(" ")
//         .map(|v| v.parse().unwrap());
//     let mut nums = Vec::new();
//     while let Some(num) = input_nums.next() {
//         println!("{}", num);
//         let mut range = (num..num + input_nums.next().unwrap()).collect();
//         nums.append(&mut range);
//     }
//     nums
// }

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
