use itertools::Itertools;
use std::fs;

pub fn read_input<'a>(year: isize, day: isize, num: isize) -> Vec<String> {
    let input_type = if num > 0 {
        "data"
    } else {
        let num = -num;
        "example"
    };
    let file_path = String::from(
        (year.to_string()) + "/" + &*day.to_string() + "/" + input_type + &*num.to_string() + ".txt",
    );
    let contents = fs::read_to_string(&file_path)
        .expect(&format!("File {} does not exist but it should!", file_path))
        .trim()
        .split('\n')
        .map(String::from)
        .collect_vec();
    contents
}
