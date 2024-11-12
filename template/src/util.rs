use itertools::Itertools;
use std::fs;

pub fn read_input<'a>(mut num: isize) -> Vec<String> {
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
