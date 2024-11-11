use itertools::Itertools;

mod util;

fn run_part1<'a>(input_str: Vec<String>) -> &'a str {
    let res = input_str;
    res.iter().concat()
}

fn run_part2<'a>(input_str: Vec<String>) -> &'a str {
    let res = input_str;
    res.iter().concat()
}

fn run_part3<'a>(input_str: Vec<String>) -> &'a str {
    let res = input_str;
    res.iter().concat()
}

fn main() {
    let input_str = util::read_input(YEAR, DAY, -1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(YEAR, DAY, 1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(YEAR, DAY, -2);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(YEAR, DAY, 2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(YEAR, DAY, -3);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(YEAR, DAY, 3);
    println!("{}", run_part3(input_str));
}
