use everybody_codes_util as util;

fn process_input(input_str: Vec<String>) -> isize {
    input_str[0].parse().unwrap()
}

fn run_part1(input_str: Vec<String>) -> String {
    let notes = process_input(input_str);
    let mut cum_sum = 0;
    let mut i = 0;
    let mut width;
    loop {
        i += 1;
        width = 2 * i - 1;
        cum_sum += width;
        if cum_sum >= notes {
            break
        }
    }
    ((cum_sum - notes) * width).to_string()
}

fn run_part2(input_str: Vec<String>, nr_acolytes: isize, nr_blocks: isize) -> String {
    let notes = process_input(input_str);
    let mut cum_sum = 0;
    let mut i = 0;
    let mut thickness = 1;
    let mut width;
    loop {
        i += 1;
        width = 2 * i - 1;
        thickness = if i == 1 {
            1
        } else {
            (thickness * notes) % nr_acolytes
        };
        cum_sum += width * thickness;
        if cum_sum >= nr_blocks {
            break
        }
    }
    ((cum_sum - nr_blocks) * width).to_string()
}

fn run_part3(input_str: Vec<String>, nr_acolytes: isize, nr_blocks: isize) -> String {
    let notes = process_input(input_str);
    let mut i = 0;
    let mut thickness = 1;
    let mut heights = Vec::new();
    let mut this_sum;
    let mut to_remove ;
    loop {
        i += 1;
        thickness = if i == 1 {
            1
        } else {
            ((thickness * notes) % nr_acolytes) + nr_acolytes
        };
        for i in 0..heights.len() {
            heights[i] += thickness
        }
        heights.push(thickness);

        let width = 2 * i - 1;
        to_remove = if i == 1 {
            0
        } else {
            (heights[0] * width * notes) % nr_acolytes
        };
        for i in 1..heights.len() - 1 {
            to_remove += 2 * ((heights[i] * width * notes) % nr_acolytes);
        }

        this_sum = heights[0] + heights.iter().skip(1).sum::<isize>() * 2;
        if this_sum - to_remove >= nr_blocks {
            break
        }
    }
    let diff = this_sum - to_remove - nr_blocks;

    diff.to_string()
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part1(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = util::read_input(-2);
    println!("Example: {}", run_part2(input_str, 5, 50));
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str, 1111, 20240000));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3(input_str, 5, 160));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str, 10, 202400000));
}
