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
    // Plan:
    // - Automatically iterate over all three parts
    // - Adjust to make it clearer that negative int is example
    // - Create nice output print function, printing part and whether it is example or actual
    // - Create template for a joint function that solves all 3

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
