use counter::Counter;
use everybody_codes_util as util;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use everybody_codes_util::grid::{Grid, Point};

fn process_input(input_str: Vec<String>) -> HashMap<String, Vec<isize>> {
    let mut this_map = HashMap::new();
    for row in input_str.iter() {
        let (key, val) = row.split(":").collect_tuple().unwrap();
        this_map.insert(
            key.to_string(),
            val.split(',')
                .map(|x| match x {
                    "+" => 1,
                    "-" => -1,
                    "=" => 0,
                    _ => {
                        println!("Should not happen");
                        0
                    }
                })
                .collect_vec(),
        );
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
    let res = res
        .iter()
        .map(|x| (x.0, apply_procedure(x.1, 10)))
        .sorted_by_key(|x| x.1)
        .map(|x| x.0)
        .rev()
        .join("");
    res
}

fn get_num(el: char) -> isize {
    match el {
        // 'S' => 0,
        '=' => 0,
        '+' => 1,
        '-' => -1,
        _ => {
            println!("Should not happen");
            0
        }
    }
}

fn process_track(race_str: Vec<String>) -> Vec<isize> {
    let mut res = Vec::new();
    let dims = (race_str.len(), race_str[0].len());
    for el in race_str[0].chars().skip(1) {
        res.push(get_num(el));
    }
    for el in race_str[1..dims.0 - 1]
        .iter()
        .map(|x| x.chars().last().unwrap())
    {
        res.push(get_num(el));
    }
    for el in race_str[dims.0 - 1].chars().rev() {
        res.push(get_num(el));
    }
    for el in race_str[1..dims.0 - 1]
        .iter()
        .map(|x| x.chars().nth(0).unwrap())
        .rev()
    {
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
    let res = res
        .iter()
        .map(|x| (x.0, run_track(x.1, &track.clone(), 10)))
        .sorted_by_key(|x| x.1)
        .map(|x| x.0)
        .rev()
        .join("");
    res
}

fn get_cands(curr: (isize, isize)) -> Vec<(isize, isize)> {
    let cands = vec![
        (curr.0, curr.1 + 1),
        (curr.0, curr.1 - 1),
        (curr.0 + 1, curr.1),
        (curr.0 - 1, curr.1),
    ];
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
            if cand.0 >= dims.0 as isize {
                continue;
            }
            if cand.0 < 0 {
                continue;
            }
            if cand.1 >= race_str[cand.0 as usize].len() as isize {
                continue;
            }
            if cand.1 < 0 {
                continue;
            }
            if !hist.contains(&cand) {
                let el = race_str[cand.0 as usize]
                    .chars()
                    .nth(cand.1 as usize)
                    .unwrap_or(' ');
                if el != ' ' {
                    curr = cand;
                    res.push(get_num(el));
                    continue 'while_lab;
                }
            }
        }
        stop = true;
    }
    res.push(0);
    res
}

fn process_advanced_track_as_grid(race_str: Vec<String>) -> Vec<isize> {
    let map: HashMap<u8, isize> = [(b'S', 0), (b'+', 1), (b'=', 0), (b'-', -1)].into_iter().collect();
    let mut grid: Grid<u8> = Grid::from(race_str.iter().map(|x| x.as_str()).collect_vec());
    &grid.fill_lines(b' ');
    let mut curr = Point::new([0, 0]);
    let mut stop = false;
    let mut res = Vec::new();
    'while_lab: while !stop {
        let neighbors: Vec<(Point<isize, 2>, u8)> = grid.get_neighbors_ok(curr);
        for (pt, val) in neighbors.iter() {
            if *val != b' ' {
                grid.set_pt(b' ', curr);
                curr = *pt;
                res.push(map[val]);
                continue 'while_lab
            }
        }
        stop = true;
    }
    res.push(0);
    res

}

fn get_options(options: Counter<isize, isize>) -> Vec<Vec<isize>> {
    if options.total::<isize>() == 1 {
        return vec![vec![options.most_common_ordered()[0].0]];
    };
    let mut this_vec = Vec::new();
    for this in options.keys() {
        if options[this] <= 0 {
            continue;
        }
        let mut this_counter = options.clone();
        this_counter[this] -= 1;
        let mut this_res = get_options(this_counter);
        for i in 0..this_res.len() {
            let mut i_res = this_res[i].clone();
            i_res.push(*this);
            this_res[i] = i_res;
        }
        this_vec.extend(this_res)
    }
    this_vec
}

fn run_part3(input_str: Vec<String>, race_str: Vec<String>) -> String {
    let rival = process_input(input_str);
    let track = process_advanced_track_as_grid(race_str);
    let rival_count = run_track(rival.get("A").unwrap(), &track, 2024);
    let options: Vec<isize> = vec![1, 1, 1, 1, 1, 0, 0, 0, -1, -1, -1];

    let mut count_win = 0;

    let options_counter = options
        .clone()
        .into_iter()
        .collect::<Counter<isize, isize>>();
    let iter_opt = get_options(options_counter);
    // Or: let iter_opt = options.iter().permutations(11).unique();

    for this_opt in iter_opt {
        let this: Vec<isize> = this_opt.into_iter().map(|x| x.clone()).collect_vec();
        let this_count = run_track(&this, &track, 2024);
        if this_count > rival_count {
            count_win += 1
        }
    }

    println!("Rival: {}", rival_count);
    count_win.to_string()
}

fn main() {
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
