use everybody_codes_util as util;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    let res = input_str;
    res.join("") + "TODO!"
}

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let res = input_str;
    res.join("") + "TODO!"
}

fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    let res = input_str;
    res.join("") + "TODO!"
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
    println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example: {}", util::run(run_part3, -3));
    println!("Actual: {}\n", util::run(run_part3, 3));

}
