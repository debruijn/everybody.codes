use itertools::Itertools;
use everybody_codes_util as util;
use counter::Counter;

fn process_input(input_str: Vec<String>) -> Vec<Vec<usize>> {
    let first_pass = input_str.iter().map(|x| x.split(' ').map(|y| y.parse::<usize>().unwrap()).collect_vec()).collect_vec();
    let mut second_pass: Vec<Vec<usize>> = Vec::new();

    for _ in first_pass[0].iter() {
        second_pass.push(Vec::new());
    }

    for row in first_pass.iter() {
        for (j, col) in row.iter().enumerate() {
            second_pass[j].push(*col)
        }
    }
    second_pass
}

fn get_shout(pattern: &Vec<Vec<usize>>) -> String {
    pattern.iter().map(|x| x[0].to_string()).join("")
}

fn run_part1(input_str: Vec<String>) -> String {
    let mut pattern = process_input(input_str);

    let nr_rounds: usize = 10;
    let mut clap_col: usize = 0;
    for _ in 0..nr_rounds {
        let next_col = (clap_col + 1) % pattern.len();
        let next_len = pattern[next_col].len();

        // Find index of clapper in next col
        let clapper = pattern[clap_col][0];
        pattern[clap_col].remove(0);
        let clap_loc = (clapper - 1) % (2 * next_len);

        // Put at right spot
        if clap_loc < next_len {
            pattern[next_col].insert(clap_loc, clapper)
        } else {
            pattern[next_col].insert(next_len - (clap_loc % next_len), clapper)
        }

        clap_col = next_col;
    }
    get_shout(&pattern)
}

fn run_part2(input_str: Vec<String>) -> String {
    let mut pattern = process_input(input_str);

    let mut clap_col: usize = 0;
    let mut counter: Counter<String, usize> = Counter::new();
    loop {
        let next_col = (clap_col + 1) % pattern.len();
        let next_len = pattern[next_col].len();

        // Find index of clapper in next col
        let clapper = pattern[clap_col][0];
        pattern[clap_col].remove(0);
        let clap_loc = (clapper - 1) % (2 * next_len);

        // Put at right spot
        if clap_loc < next_len {
            pattern[next_col].insert(clap_loc, clapper)
        } else {
            pattern[next_col].insert(next_len - (clap_loc % next_len), clapper)
        }

        // Shout
        let this_shout = get_shout(&pattern);
        counter[&this_shout] += 1;
        clap_col = next_col;
        if counter[&this_shout] == 2024 {
            return (this_shout.parse::<usize>().unwrap() * counter.total::<usize>()).to_string()
        }
    }
}

fn run_part3(input_str: Vec<String>) -> String {
    // Brute force solution. Better approaches:
    // - Something theoretical about which dancers can join up together to shout
    // - Find repetition in the shout -> based on position, not on number (since numbers are reused)
    // - Calculate when it should start repeating at most
    let mut pattern = process_input(input_str);
    let mut max_shout = 0;
    let mut clap_col: usize = 0;
    let nr_rounds = 10000;  // First did way more, but this was enough for my data

    for _ in 0..nr_rounds {
        let next_col = (clap_col + 1) % pattern.len();
        let next_len = pattern[next_col].len();

        // Find index of clapper in next col
        let clapper = pattern[clap_col][0];
        pattern[clap_col].remove(0);
        let clap_loc = (clapper - 1) % (2 * next_len);

        // Put at right spot
        if clap_loc < next_len {
            pattern[next_col].insert(clap_loc, clapper)
        } else {
            pattern[next_col].insert(next_len - (clap_loc % next_len), clapper)
        }

        // Shout
        let this_shout = get_shout(&pattern).parse::<u128>().unwrap();
        clap_col = next_col;
        if this_shout > max_shout {
            max_shout = this_shout
        }
    }
    max_shout.to_string()
}

fn main() {
    // Could clean up by combining the dance/clap routine in a special dance func

    // Part 1: example and actual
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part1(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = util::read_input(-2);
    println!("Example: {}", run_part2(input_str));
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));
}
