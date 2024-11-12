use itertools::Itertools;

mod util;

fn run_part1(input_str: Vec<String>) -> String {
    let res = input_str;
    res.join("")
}

fn run_part2(input_str: Vec<String>) -> String {
    let res = input_str;
    res.join("")
}

fn run_part3<'a>(input_str: Vec<String>) -> String {
    let res = input_str;
    res.join("")
}

fn main() {
    let input_str = util::read_input(-1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(-2);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(-3);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(3);
    println!("{}", run_part3(input_str));
}
