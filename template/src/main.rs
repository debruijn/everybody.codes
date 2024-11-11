use itertools::Itertools;

mod util;

fn run<'a>(input_str: Vec<String>) -> &'a str {
    let res = input_str;
    res.iter().concat()
}

fn main() {
    let input_str = util::read_input(YEAR, DAY);
    println!("{}", run(input_str))
}
