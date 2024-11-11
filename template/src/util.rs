use itertools::Itertools;
use std::fs;

pub fn read_input<'a>(year: isize, day: isize) -> Vec<String> {
    let file_path = String::from(
        "input/".to_owned() + &*year.to_string() + "_" + &format!("{:02}", day) + "_input.txt",
    );
    let contents = fs::read_to_string(&file_path)
        .expect(&format!("File {} does not exist but it should!", file_path))
        .trim()
        .split('\n')
        .map(String::from)
        .collect_vec();
    contents
}
