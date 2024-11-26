use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use everybody_codes_util as util;
use counter::Counter;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
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
                // println!("{:?}", vecs[j])
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

pub fn lcm(nums: &[isize]) -> isize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: isize, b: isize) -> isize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let turns: Vec<usize> = input_str[0].split(',').map(|x| x.parse().unwrap()).collect_vec();

    let mut vecs: Vec<VecDeque<String>> = Vec::new();

    // TODO: to improve, don't even process the snout..
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


    // let target_num = 202420242024;
    let target_num = 202420242024;
    let mut top_row: String;
    let mut hist: Vec<String> = Vec::new();
    let mut points: Vec<usize> = Vec::new();

    // let mut cycle_len: BigInt = BigInt::one();
    // let mut cycle_len: usize = 1;
    // for num in vecs.iter().enumerate().map(|x| x.1.len() * turns[x.0]) {
    //     cycle_len = cycle_len.lcm(&(&num.into()));
    // }
    let cycle_len = lcm(&*vecs.iter().enumerate().map(|x| x.1.len() as isize * (turns[x.0] as isize % x.1.len() as isize)).collect_vec());

    let target_rem = target_num % cycle_len;

    println!("{}", cycle_len);

    let mut counter = 0;
    'outer: for _i in 0..cycle_len {
        counter += 1;
        for (j, wheel) in vecs.iter_mut().enumerate(){
            wheel.rotate_left(turns[j] % wheel.len());
        }
        top_row = vecs.iter().map(|x| x[0].clone()).join(" ");
        // if hist.contains(&top_row) {
        //     break 'outer
        // }
        let count: Counter<char, usize> = Counter::from_iter(top_row.chars().step_by(2));
        // let count: Counter<char, usize> = Counter::from_iter(top_row.replace(" ", "").chars().step_by(2));
        hist.push(top_row.clone());
        let mut pt_sum = 0;
        for x in count.values() {
            if *x > 2 {
                pt_sum += *x - 2usize;
            }
        }
        points.push(pt_sum);
    }
    let left = target_num - counter;
    // let cycle_len = counter;
    let cycle_count = left / cycle_len + 1;
    let left_rem = left % cycle_len;

    let total: usize =
        cycle_count as usize * points.get(..).unwrap_or_default().into_iter().sum::<usize>() +
        points.get(..left_rem as usize).unwrap_or_default().into_iter().sum::<usize>();


    total.to_string()
}


fn get_val(offset: &Vec<isize>, data: &Vec<Vec<String>>, steps: &Vec<isize>, i: isize, cycle_len: isize) -> (isize, Vec<isize>) {
    // For a particular offset from the starting point, get the points and the new offset
    let this_offset = offset.iter().enumerate().map(|x| (steps[x.0] + x.1 + i + cycle_len) % cycle_len).collect_vec();
    let this_row = data.iter().enumerate().map(|x| x.1[this_offset[x.0] as usize % x.1.len()].clone()).join("");

    // let count: Counter<char, usize> = Counter::from_iter(this_row.chars().step_by(2));
    let count: Counter<char, usize> = Counter::from_iter(this_row.chars());
    let mut pt_sum = 0;
    for x in count.values() {
        if *x > 2 {
            pt_sum += *x - 2;
        }
    }
    (pt_sum as isize, this_offset)
}

// map: &mut HashMap<(usize, usize), (usize, usize)>
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

    // Start the recursive algorithm
    let cycle_len = lcm(&*vecs.iter().enumerate().map(|x| x.1.len() as isize * (turns[x.0] % x.1.len() as isize)).collect_vec());
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
    // println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example 1: {}", util::run(run_part3, -3));
    println!("Actual: {}\n", util::run(run_part3, 3));

}
