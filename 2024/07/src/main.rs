use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use everybody_codes_util as util;

fn process_input(input_str: Vec<String>) -> HashMap<String, Vec<isize>> {
    let mut this_map = HashMap::new();
    for row in input_str.iter() {
        let (key, val) = row.split(":").collect_tuple().unwrap();
        this_map.insert(key.to_string(), val.split(',').map(|x| match x {
            "+" => 1,
            "-" => -1,
            "=" => 0,
            _ => {println!("Should not happen"); 0}
        }).collect_vec());
    }
    this_map
}

fn apply_procedure(steps: &Vec<isize>, nr_steps: isize) -> isize {
    let mut val = 10;
    let mut cum_val = 0;
    let mut cycle = steps.iter().cycle();
    for _ in 0..nr_steps {
        val += cycle.next().unwrap();
        cum_val += val;
    }

    cum_val
}

fn run_part1(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let res = res.iter().map(
        |x| (x.0, apply_procedure(x.1, 10))).
        sorted_by_key(|x| x.1).map(|x| x.0).rev().join("");
    res
}

fn get_num(el: char) -> isize {
    match el {
        // 'S' => 0,
        '=' => 0,
        '+' => 1,
        '-' => -1,
        _ => {println!("Should not happen"); 0}
    }
}

fn process_track(race_str: Vec<String>) -> Vec<isize> {
    let mut res = Vec::new();
    let dims = (race_str.len(), race_str[0].len());
    for el in race_str[0].chars().skip(1) {
        res.push(get_num(el));
    }
    for el in race_str[1..dims.0-1].iter().map(|x| x.chars().last().unwrap()) {
        res.push(get_num(el));
    }
    for el in race_str[dims.0-1].chars().rev() {
        res.push(get_num(el));
    }
    for el in race_str[1..dims.0-1].iter().map(|x| x.chars().nth(0).unwrap()).rev() {
        res.push(get_num(el));
    }
    res.push(0);
    res
}

fn run_track(steps: &Vec<isize>, track: &Vec<isize>, nr_loops: isize) -> isize {
    let mut val = 10;
    let mut cum_val = 0;
    let nr_steps = nr_loops * track.len() as isize;
    let mut cycle = steps.iter().cycle();
    let mut track_cycle = track.iter().cycle();
    for _ in 0..nr_steps {
        let next_val = cycle.next().unwrap();
        let next_track = track_cycle.next().unwrap();
        if *next_track == 0 {
            val += next_val;
        } else {
            val += next_track;
        }
        cum_val += val;
    }
    cum_val
}

fn run_part2(input_str: Vec<String>, race_str: Vec<String>) -> String {
    let res = process_input(input_str);
    let track = process_track(race_str);
    let res = res.iter().map(
        |x| (x.0, run_track(x.1, &track.clone(), 10))).
        sorted_by_key(|x| x.1).map(|x| x.0).rev().join("");
    res
}

fn get_cands(curr: (isize, isize)) -> Vec<(isize, isize)> {
    let cands = vec!((curr.0, curr.1+1),
                         (curr.0, curr.1-1),
                         (curr.0+1, curr.1),
                         (curr.0-1, curr.1));
    cands
}

fn process_advanced_track(race_str: Vec<String>) -> Vec<isize> {
    let mut res = Vec::new();
    let dims = (race_str.len(), race_str[0].len());
    let mut hist = HashSet::new();
    let mut curr = (0, 0);
    // First is 0,1. Keep looking until back at S.
    let mut stop = false;
    'while_lab: while !stop {
        hist.insert(curr);
        for cand in get_cands(curr) {
            if cand.0 >= dims.0 as isize {continue}
            if cand.0 < 0 {continue}
            if cand.1 >= race_str[cand.0 as usize].len() as isize {continue}
            if cand.1 < 0 {continue}
            if !hist.contains(&cand) {
                let el = race_str[cand.0 as usize].chars().nth(cand.1 as usize).unwrap_or(' ');
                if el != ' ' {
                    curr = cand;
                    res.push(get_num(el));
                    continue 'while_lab
                }
            }
        }
        stop = true;
    }
    res.push(0);
    res
}

fn run_part3(input_str: Vec<String>, race_str: Vec<String>) -> String {
    let rival =  process_input(input_str);
    let track = process_advanced_track(race_str);
    // Count score rival
    let rival_count = run_track(rival.get("A").unwrap(), &track, 2024);
    // Loop over all options (itertools?) and note number of winnings
    let options: Vec<isize> = vec!(1,1,1,1,1,0,0,0,-1,-1,-1);
    let mut count_win = 0;
    for this_opt in options.iter().permutations(11).unique() {
        let this: Vec<isize> = this_opt.into_iter().map(|x| x.clone()).collect_vec();
        let this_count = run_track(&this, &track, 2024);
        if this_count > rival_count {count_win+=1}
    }

    println!("Rival: {}", rival_count);
    count_win.to_string()
}

fn main() {
    // Plan:
    // - Automatically iterate over all three parts
    // - Adjust to make it clearer that negative int is example
    // - Create nice output print function, printing part and whether it is example or actual
    // - Create template for a joint function that solves all 3

    // Part 1: example and actual
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part1(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = util::read_input(-1);
    let race_str = util::read_input(-2);
    println!("Example: {}", run_part2(input_str, race_str));
    let input_str = util::read_input(2);
    let race_str = util::read_input(3);
    println!("Actual: {}\n", run_part2(input_str, race_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(4);
    let race_str = util::read_input(5);
    println!("Actual: {}\n", run_part3(input_str, race_str));

}
