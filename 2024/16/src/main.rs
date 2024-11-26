use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use everybody_codes_util as util;
use counter::Counter;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    // In part 1 I actually do the steps without figuring out how to avoid doing the steps :)
    // VedDeque is used for rotating purposes.
    let turns: Vec<usize> = input_str[0].split(',').map(|x| x.parse().unwrap()).collect_vec();
    let mut vecs: Vec<VecDeque<String>> = Vec::new();

    for (i, row) in input_str[2..].iter().enumerate() {
        let mut iter_vec = Vec::new();
        for col in &row.chars().chunks(4) {
            iter_vec.push(col.filter(|x| *x != ' ').join(""));
        };
        for (j, col) in iter_vec.into_iter().enumerate() {
            if i==0 {
                vecs.push(VecDeque::new());
            }
            if col.len()>0 {
                vecs[j].push_back(col);
            }
        }
    }

    for _ in 0..100 {
        for (j, wheel) in vecs.iter_mut().enumerate(){
            wheel.rotate_left(turns[j] % wheel.len());
        }
    }

    let res = vecs.iter().map(|x| x[0].clone()).join(" ");
    res
}


fn get_vecs(input_str: Vec<String>) -> (Vec<isize>, Vec<Vec<String>>, isize) {
    // Process input to relevant criteria:
    // - vec of turns to take for each wheel
    // - the vec of 3-char strings for each wheel
    // - the cycle length after which everything is guaranteed to repeat
    let turns: Vec<isize> = input_str[0].split(',').map(|x| x.parse().unwrap()).collect_vec();
    let mut vecs: Vec<Vec<String>> = Vec::new();

    // Sort-of transpose the data. Work around chunks not being an iterator but usable in for-loop.
    for (i, row) in input_str[2..].iter().enumerate() {
        let mut iter_vec = Vec::new();
        for col in &row.chars().chunks(4) {
            iter_vec.push(col.filter(|x| *x != ' ').join(""));
        };
        for (j, col) in iter_vec.into_iter().enumerate() {
            if i==0 {
                vecs.push(Vec::new());
            }
            if col.len()>0 {
                vecs[j].push(col.chars().step_by(2).join(""));
            }
        }
    }
    let cycle_len = util::lcm(&*vecs.iter().enumerate()
        .map(|x| x.1.len() as isize)
        .collect_vec());

    (turns, vecs, cycle_len)
}



fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let (turns, vecs, cycle_len) = get_vecs(input_str);
    let target_num = 202420242024;
    let mut points: Vec<isize> = Vec::new();
    let mut offset = vec![0;turns.len()];
    let mut counter = 0;
    let mut temp_sum: isize;
    for _i in 0..cycle_len {
        (temp_sum, offset) = get_val(&offset, &vecs, &turns, 0, cycle_len);
        counter += 1;
        points.push(temp_sum);
    }
    util::extrapolate_cumulative_cycle(0, counter, target_num, points).to_string()
}


fn get_val(offset: &Vec<isize>, data: &Vec<Vec<String>>, steps: &Vec<isize>, i: isize, cycle_len: isize) -> (isize, Vec<isize>) {
    // For a particular offset from the starting point, get the points and the new offset
    let this_offset = offset.iter().enumerate().map(|x| (steps[x.0] + x.1 + i + cycle_len) % cycle_len).collect_vec();
    let this_row = data.iter().enumerate().map(|x| x.1[this_offset[x.0] as usize % x.1.len()].clone()).join("");

    let count: Counter<char, usize> = Counter::from_iter(this_row.chars());
    let mut pt_sum = 0;
    for x in count.values() {
        if *x > 2 {
            pt_sum += *x - 2;
        }
    }
    (pt_sum as isize, this_offset)
}

fn get_minmax<'a>(offset: Vec<isize>, n_pull: isize, data: &Vec<Vec<String>>, steps: &Vec<isize>,
              cycle_len: isize, map: &mut HashMap<(Vec<isize>, isize), (isize, isize)>) -> (isize, isize) {
    // For a particular offset from the starting point and number of pulls remaining, get the min
    // and max points that can be achieved from that point:
    // - If this input has not been estimated, calculate it recursively from all options for next
    //   pull. Store the result in a HashMap.
    // - If this input has already been estimated, return that.

    if map.contains_key(&(offset.clone(), n_pull)) {  // Already done
        map[&(offset, n_pull)]
    } else {  // New input
        let mut res = Vec::new();
        for i in -1..2 { // push back, stay, or push forward
            let (i_val, i_offset) = get_val(&(offset.clone()), data, steps, i, cycle_len);
            if n_pull > 1 {
                let (i_min, i_max) = get_minmax(i_offset, n_pull - 1, data, steps, cycle_len, map);
                res.push((i_min + i_val, i_max + i_val))
            } else {
                res.push((i_val, i_val));
            }
        }
        let res = (res.iter().map(|x| x.0).min().unwrap(),
                   res.iter().map(|x| x.1).max().unwrap());
        map.insert((offset, n_pull), res);
        res
    }
}

fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    // Process input
    let mut map: HashMap<(Vec<isize>, isize), (isize, isize)> = HashMap::new();  // offset, pulls left
    let (turns, vecs, cycle_len) = get_vecs(input_str);

    // Start the recursive algorithm
    let minmax = get_minmax(vec![0;turns.len()], 256, &vecs, &turns, cycle_len, &mut map);

    format!("{} {}", minmax.1, minmax.0)
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    util::run(run_part1, -1);  // Included because first call is slower for allocation reasons
    util::run(run_part1, 1);
    println!("Example: {}", util::run(run_part1, -1));
    println!("Actual: {}\n", util::run(run_part1, 1));

    // Part 2: example and actual
    println!("Part 2");
    println!("Example: {}", util::run(run_part2, -2));
    println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example 1: {}", util::run(run_part3, -3));
    println!("Actual: {}\n", util::run(run_part3, 3));

}
