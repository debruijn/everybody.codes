pub mod grid;
pub mod nonhashmap;

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

pub fn run(f: &dyn Fn(Vec<String>, bool) -> String, file: isize) -> String {
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
