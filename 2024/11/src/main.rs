use counter::Counter;
use itertools::Itertools;
use std::collections::HashMap;

fn run_part1(input_str: Vec<&str>) -> String {
    run_process(&input_str, "A", 4).to_string()
}

fn run_part2(input_str: Vec<&str>) -> String {
    run_process(&input_str, "Z", 10).to_string()
}

fn run_process(input_str: &Vec<&str>, start: &str, n_days: usize) -> usize {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for row in input_str {
        let (key, val) = row.split(':').collect_tuple().unwrap();
        let val: Vec<&str> = val.split(',').collect_vec();
        map.insert(key, val);
    }
    let mut curr: Counter<&&str, usize> = Counter::new();
    curr += [&start; 1];

    for _ in 0..n_days {
        let mut new: Counter<&&str, usize> = Counter::new();
        for (k, v) in curr.iter() {
            let this: Counter<&&str, usize> = Counter::from_iter(&map[*k]);
            for k2 in this.keys() {
                new[k2] += &this[k2] * v
            }
        }
        curr = new;
    }

    let res: usize = curr.total();
    res
}

fn run_process_string(input_str: &Vec<&str>, start: &str, n_days: usize) -> usize {
    let mut map_counter: HashMap<String, Counter<String, usize>> = HashMap::new();
    for row in input_str {
        let (key, val) = row.split(':').collect_tuple().unwrap();
        let val: Vec<String> = val.split(',').map(|x| x.to_string()).collect_vec();
        map_counter.insert(key.to_string(), Counter::from_iter(val));
    }
    let mut curr: Counter<String, usize> = Counter::new();
    curr += [start.to_string(); 1];

    for _ in 0..n_days {
        let mut new: Counter<String, usize> = Counter::new();
        for (k, v) in curr.iter() {
            let this: Counter<String, usize> = map_counter[k].clone();
            for (k2, v2) in this.iter() {
                new[k2] += v2 * v;
            }
        }
        curr = new;
    }

    let res: usize = curr.total();
    res
}

fn run_part3(input_str: Vec<&str>) -> String {
    let input_clone = input_str.clone();
    let keys: Vec<&str> = input_str
        .iter()
        .map(|x| x.split(':').collect_vec()[0])
        .collect_vec();

    let first = run_process_string(&input_clone, keys[0], 20);
    let mut min = first;
    let mut max = first;

    for key in keys.iter().skip(1) {
        let this = run_process(&input_clone, key, 20);
        if this < min {
            min = this
        }
        if this > max {
            max = this
        }
    }

    (max - min).to_string()
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    let input_str = include_str!("../example1.txt").split('\n').collect_vec();
    println!("Example: {}", run_part1(input_str));
    let input_str = include_str!("../data1.txt").split('\n').collect_vec();
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = include_str!("../data2.txt").split('\n').collect_vec();
    println!("Actual: {}\n", run_part2(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = include_str!("../example3.txt").split('\n').collect_vec();
    println!("Example: {}", run_part3(input_str));
    let input_str = include_str!("../data3.txt").split('\n').collect_vec();
    println!("Actual: {}\n", run_part3(input_str));
}
