use counter::Counter;
use everybody_codes_util as util;
use itertools::Itertools;
use std::collections::HashMap;

fn process_input(input_str: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut this_map = HashMap::new();
    for row in input_str.iter() {
        let (key, value) = row.split(":").collect_tuple().unwrap();
        this_map.insert(
            key.to_string(),
            value.split(",").map(|x| x.to_string()).collect_vec(),
        );
    }
    this_map
}

fn run_part1(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let paths = get_applemap(&res, vec!["RR".parse().unwrap()], "RR".parse().unwrap());
    let counter = paths
        .iter()
        .map(|x| x.len())
        .collect::<Counter<usize, usize>>();
    let path_len = counter.iter().min_by_key(|x| x.1).unwrap().0;
    let powerful_path = paths.iter().filter(|x| x.len() == *path_len).collect_vec()[0];
    powerful_path.join("")
}

fn get_applemap(
    tree: &HashMap<String, Vec<String>>,
    curr_path: Vec<String>,
    curr_loc: String,
) -> Vec<Vec<String>> {
    let mut this_paths = Vec::new();
    for new in tree[&curr_loc].iter() {
        if new == "BUG" || new == "ANT" {
            continue;
        }
        if new == "@" {
            let mut new_path = curr_path.clone();
            new_path.push(String::from("@"));
            this_paths.push(new_path);
            continue;
        }
        if tree.keys().contains(new) {
            let mut new_path = curr_path.clone();
            new_path.push(new.to_string());
            let new_map = get_applemap(tree, new_path, new.to_string());
            this_paths.extend(new_map)
        }
    }
    this_paths
}

fn run_part2(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let paths = get_applemap(&res, vec!["RR".parse().unwrap()], "RR".parse().unwrap());
    let counter = paths
        .iter()
        .map(|x| x.len())
        .collect::<Counter<usize, usize>>();
    let path_len = counter.iter().min_by_key(|x| x.1).unwrap().0;
    let powerful_path = paths.iter().filter(|x| x.len() == *path_len).collect_vec()[0];
    let first_letter_path = powerful_path
        .iter()
        .map(|x| x.chars().nth(0).unwrap())
        .join("");
    first_letter_path
}

fn run_part3(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let paths = get_applemap(&res, vec!["RR".parse().unwrap()], "RR".parse().unwrap());
    let counter = paths
        .iter()
        .map(|x| x.len())
        .collect::<Counter<usize, usize>>();
    let path_len = counter.iter().min_by_key(|x| x.1).unwrap().0;
    let powerful_path = paths.iter().filter(|x| x.len() == *path_len).collect_vec()[0];
    let first_letter_path = powerful_path
        .iter()
        .map(|x| x.chars().nth(0).unwrap())
        .join("");
    first_letter_path
}

fn main() {
    // Part 1: example and actual  RRCQPX@
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part1(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));
}
