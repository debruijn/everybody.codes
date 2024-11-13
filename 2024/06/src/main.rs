use std::collections::HashMap;
use itertools::Itertools;
use everybody_codes_util as util;

fn process_input(input_str: Vec<String>) -> HashMap<String,Vec<String>> {
    let mut this_map = HashMap::new();
    for row in input_str.iter() {
        let (key, value) = row.split(":").collect_tuple().unwrap();
        this_map.insert(key.to_string(), value.split(",").map(|x| x.to_string()).collect_vec());
    }
    this_map
}

fn get_paths<'a, 'b, 'c>(tree: &'a HashMap<String, Vec<String>>, curr_path: String, curr_len: usize, curr_loc: String) -> (String, usize, bool){

    // println!("{}, {}, {}", curr_path, curr_len, curr_loc);
    let mut this_paths = Vec::new();
    for new in tree[&curr_loc].iter() {
        if new == "@" {
            println!("{}, {}", curr_path.parse::<String>().unwrap() + "@", curr_len);
            return (curr_path.parse::<String>().unwrap() + "@", curr_len, true)
        } else {
            if new == "BUG" || new == "ANT" {
                continue
            }
            if tree.keys().contains(new) {
                let (new_path, new_len, new_bool) = get_paths(tree, (curr_path.to_string() + new), curr_len + 1, new.to_string());
                this_paths.push((new_path, new_len, new_bool))
                }
        }
    }
    if this_paths.iter().filter(|x| x.2).collect_vec().len() > 0 {
        this_paths.iter().filter(|x| x.2).min_by_key(|x| x.1).unwrap().to_owned()
    } else {
        (String::new(), 99, false)
    }
}


fn run_part1(input_str: Vec<String>) -> String {


    let res = process_input(input_str);
    println!("{:?}", res);

    format!("{:?}", get_paths(&res, "RR".parse().unwrap(), 0, "RR".parse().unwrap()))
}

fn get_applemap<'a, 'b, 'c>(tree: &'a HashMap<String, Vec<String>>, curr_path: Vec<String>, curr_len: usize, curr_loc: String) -> Vec<Vec<String>>{

    // println!("{}, {}, {}", curr_path, curr_len, curr_loc);
    let mut this_paths = Vec::new();
    for new in tree[&curr_loc].iter() {
        if new == "@" {
            // println!("{}, {}", curr_path.parse::<String>().unwrap() + "@", curr_len);
            let mut new_path = curr_path.clone();
            new_path.push(String::from("@"));
            this_paths.push(new_path);
        } else {
            if tree.keys().contains(new) {
                let mut new_path = curr_path.clone();
                new_path.push(new.to_string());
                let new_map = get_applemap(tree, new_path, curr_len + 1, new.to_string());
                this_paths.extend(new_map)
            }
        }
    }
    this_paths
}

fn run_part2(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let paths = get_paths(&res, "RR".parse().unwrap(), 0, "RR".parse().unwrap());
    format!("{:?}", paths)
}

fn run_part3(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let paths = get_paths(&res, "RR".parse().unwrap(), 0, "RR".parse().unwrap());
    format!("{:?}", paths)
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
