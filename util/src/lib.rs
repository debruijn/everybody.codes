pub mod grid;
pub mod nohashmap;

use itertools::{Itertools, MinMaxResult};
use std::fs;
use std::time::Instant;

pub fn minmax(min_max: MinMaxResult<isize>) -> [isize; 2] {
    match min_max {
        MinMaxResult::NoElements => [0, 0],
        MinMaxResult::OneElement(a) => [a, a],
        MinMaxResult::MinMax(a, b) => [a, b],
    }
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

pub fn extrapolate_cycle(first: usize, now: usize, target: usize, vals: Vec<isize>) -> isize {
    let remaining = target - now;
    let cycle_len = now - first;
    let remaining = remaining % cycle_len;
    let target_val = vals[first + remaining - 1];
    target_val
}

pub fn extrapolate_cumulative_cycle(first: usize, now: usize, target: usize, vals: Vec<isize>) -> isize {
    let remaining = target - now;
    let cycle_len = now - first;
    let cycle_count = (remaining + cycle_len) / cycle_len;
    let remaining = remaining % cycle_len;
    let cycle_val = vals[first..].iter().sum::<isize>();
    let first_val = vals[..first].iter().sum::<isize>();
    let remaining_val = vals[first..first + remaining].iter().sum::<isize>();
    first_val + cycle_val * cycle_count as isize + remaining_val
}



pub fn run(f: fn(Vec<String>, bool) -> String, file: isize) -> String {
    let input_str = read_input(file);
    let before = Instant::now();
    let out = f(input_str, file < 0);
    let after = Instant::now();
    format!("{} in {:?}", out, after - before)
}

pub fn read_input<'a>(mut num: isize) -> Vec<String> {
    // Can refactor to use include_str!
    let input_type = if num > 0 {
        "data"
    } else {
        num = -num;
        "example"
    };
    let file_path = String::from(input_type.to_owned() + &*num.to_string() + ".txt");
    let contents = fs::read_to_string(&file_path)
        .expect(&format!("File {} does not exist but it should!", file_path))
        .trim()
        .split('\n')
        .map(String::from)
        .collect_vec();
    contents
}
