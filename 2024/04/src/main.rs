use everybody_codes_util as util;
use itertools::Itertools;

fn process_input(input_str: Vec<String>) -> Vec<isize> {
    input_str
        .iter()
        .map(|x| x.parse::<isize>().unwrap())
        .collect_vec()
}

fn run_part12(input_str: Vec<String>) -> String {
    let inputs = process_input(input_str);
    let res: isize = inputs
        .iter()
        .map(|x| x - inputs.iter().min().unwrap())
        .sum();
    res.to_string()
}

fn nr_strikes_for_given_height(inputs: &Vec<isize>, target_height: isize) -> usize {
    inputs.iter().map(|x| x.abs_diff(target_height)).sum()
}

fn do_bisect(
    inputs: Vec<isize>,
    curr_guesses: (isize, isize),
    curr_vals: (usize, usize),
) -> (isize, usize) {
    // Take upper or lower half of interval based on comparison of interval bounds
    // This makes some assumptions and does not work for every convex function, but it works here
    // Otherwise, we would've needed two more evaluations (at 25% and 75%) and take (0 - 50),
    // (25 - 75) or (50 - 100) based on whether 25%, 50% or 75% would be the lowest.
    let mid = (curr_guesses.0 + curr_guesses.1) / 2;
    if mid == curr_guesses.0 {
        return if curr_vals.0 < curr_vals.1 {
            (curr_guesses.0, curr_vals.0)
        } else {
            (curr_guesses.1, curr_vals.1)
        };
    }
    let mid_val = nr_strikes_for_given_height(&inputs, mid);
    if curr_vals.0 < curr_vals.1 {
        do_bisect(inputs, (curr_guesses.0, mid), (curr_vals.0, mid_val))
    } else {
        do_bisect(inputs, (mid, curr_guesses.1), (mid_val, curr_vals.1))
    }
}

fn run_part3(input_str: Vec<String>) -> String {
    let inputs = process_input(input_str);
    let start = (
        inputs.iter().min().unwrap().clone(),
        inputs.iter().max().unwrap().clone(),
    );
    let start_val = (
        nr_strikes_for_given_height(&inputs, start.0),
        nr_strikes_for_given_height(&inputs, start.1),
    );

    let res = do_bisect(inputs, start, start_val);
    format!("{} strikes to get all at height {}", res.1, res.0)
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part12(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part12(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part12(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));
}
