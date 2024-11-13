use std::collections::HashMap;
use itertools::Itertools;
use everybody_codes_util as util;

fn process_input(input_str: Vec<String>) -> HashMap<(isize, isize), isize> {
    let mut locs = HashMap::new();
    for (i, row) in input_str.iter().enumerate() {
        for (j, el) in row.chars().enumerate() {
            if el == '#' {
                locs.insert((i as isize, j as isize), 1 as isize);
            }
        }
    }
    locs
}

fn take_step(mut grid: HashMap<(isize, isize), isize>, depth: isize) -> HashMap<(isize, isize), isize> { // depth: what we have now
    'el_loop: for el in grid.clone().iter() {
        let others = vec!((el.0.0 - 1, el.0.1),
                          (el.0.0 + 1, el.0.1),
                          (el.0.0, el.0.1 - 1),
                          (el.0.0, el.0.1 + 1));
        if el.1 < &depth {
            continue
        }
        for other in others {
            if !grid.keys().contains(&other) {
                continue 'el_loop;
            }
            if *grid.get(&other).unwrap_or(&0) < depth {
                continue 'el_loop;
            }
        }
        grid.insert(*el.0, el.1 + 1);
    }
    grid
}

fn run_part1(input_str: Vec<String>) -> String {
    let mut grid = process_input(input_str);
    let mut curr_sum = grid.values().sum::<isize>();
    let mut old_sum = 0;
    let mut depth = 0;
    while old_sum != curr_sum {
        depth += 1;
        old_sum = curr_sum;
        grid = take_step(grid, depth);
        curr_sum = grid.values().sum::<isize>();
    }

    grid.values().sum::<isize>().to_string()
}

fn run_part2(input_str: Vec<String>) -> String {
    let mut grid = process_input(input_str);
    let mut curr_sum = grid.values().sum::<isize>();
    let mut old_sum = 0;
    let mut depth = 0;
    while old_sum != curr_sum {
        depth += 1;
        old_sum = curr_sum;
        grid = take_step(grid, depth);
        curr_sum = grid.values().sum::<isize>();
    }

    grid.values().sum::<isize>().to_string()
}

fn take_step_diag(mut grid: HashMap<(isize, isize), isize>, depth: isize) -> HashMap<(isize, isize), isize> { // depth: what we have now
    'el_loop: for el in grid.clone().iter() {
        let others = vec!((el.0.0 - 1, el.0.1),
                          (el.0.0 + 1, el.0.1),
                          (el.0.0, el.0.1 - 1),
                          (el.0.0, el.0.1 + 1),
                          (el.0.0 - 1, el.0.1 - 1),
                          (el.0.0 + 1, el.0.1 + 1),
                          (el.0.0 + 1, el.0.1 - 1),
                          (el.0.0 - 1, el.0.1 + 1));
        if el.1 < &depth {
            continue
        }
        for other in others {
            if !grid.keys().contains(&other) {
                continue 'el_loop;
            }
            if *grid.get(&other).unwrap_or(&0) < depth {
                continue 'el_loop;
            }
        }
        grid.insert(*el.0, el.1 + 1);
    }
    grid
}

fn extend_input(mut locs: HashMap<(isize, isize), isize>) -> HashMap<(isize, isize), isize> {
    let dims = (locs.keys().map(|x| x.0).max().unwrap_or(0) + 1,
                locs.keys().map(|x| x.1).max().unwrap_or(0) + 1);
    for i in -1..dims.0+1 {
        locs.insert((i, -1), 0);
        locs.insert((i, dims.1), 0);
    }
    for j in -1..dims.1+1 {
        locs.insert((-1, j), 0);
        locs.insert((dims.0, j), 0);
    }
    locs
}


fn run_part3<'a>(input_str: Vec<String>) -> String {
    let mut grid = process_input(input_str);
    grid = extend_input(grid);
    let mut curr_sum = grid.values().sum::<isize>();
    let mut old_sum = 0;
    let mut depth = 0;
    while old_sum != curr_sum {
        depth += 1;
        println!("{}: {}", depth, curr_sum);
        old_sum = curr_sum;
        grid = take_step_diag(grid, depth);
        curr_sum = grid.values().sum::<isize>();
    }

    grid.values().sum::<isize>().to_string()
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
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(-3);
    println!("{}", run_part3(input_str));

    let input_str = util::read_input(3);
    println!("{}", run_part3(input_str));
}
