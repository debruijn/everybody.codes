use everybody_codes_util as util;
use itertools::Itertools;
use std::collections::HashMap;

fn process_input(input_str: Vec<String>) -> HashMap<(isize, isize), isize> {
    let mut locs = HashMap::new();
    for (i, row) in input_str.iter().enumerate() {
        for (j, el) in row.chars().enumerate() {
            if el == '#' {
                locs.insert((i as isize, j as isize), 1isize);
            }
        }
    }
    locs
}

fn take_step(
    mut grid: HashMap<(isize, isize), isize>,
    depth: isize,
    part: isize,
) -> HashMap<(isize, isize), isize> {
    'el_loop: for el in grid.clone().iter() {
        let mut others = vec![
            (el.0 .0 - 1, el.0 .1),
            (el.0 .0 + 1, el.0 .1),
            (el.0 .0, el.0 .1 - 1),
            (el.0 .0, el.0 .1 + 1),
        ];
        if part == 3 {
            others.extend(vec![
                (el.0 .0 - 1, el.0 .1 - 1),
                (el.0 .0 + 1, el.0 .1 + 1),
                (el.0 .0 + 1, el.0 .1 - 1),
                (el.0 .0 - 1, el.0 .1 + 1),
            ]);
        }
        if el.1 < &depth {
            continue;
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
        grid = take_step(grid, depth, 1);
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
        grid = take_step(grid, depth, 2);
        curr_sum = grid.values().sum::<isize>();
    }
    grid.values().sum::<isize>().to_string()
}

fn run_part3<'a>(input_str: Vec<String>) -> String {
    let mut grid = process_input(input_str);
    let mut curr_sum = grid.values().sum::<isize>();
    let mut old_sum = 0;
    let mut depth = 0;
    while old_sum != curr_sum {
        depth += 1;
        old_sum = curr_sum;
        grid = take_step(grid, depth, 3);
        curr_sum = grid.values().sum::<isize>();
    }
    grid.values().sum::<isize>().to_string()
}

fn run_all(input_str: Vec<String>, part: isize) -> String {
    let mut grid = process_input(input_str);
    let mut curr_sum = grid.values().sum::<isize>();
    let mut old_sum = 0;
    let mut depth = 0;
    while old_sum != curr_sum {
        depth += 1;
        old_sum = curr_sum;
        grid = take_step(grid, depth, part);
        curr_sum = grid.values().sum::<isize>();
    }
    grid.values().sum::<isize>().to_string()
}

fn main() {
    // Initial solutions: dedicated function per part

    // Part 1: example and actual
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part1(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: actual
    println!("Part 2");
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));

    // Improved solution: shared functionality that encapsulates all parts
    for i in 1..4 {
        println!("Part {}", i);
        let input_str = util::read_input(-i);
        println!("Example: {}", run_all(input_str, i));
        let input_str = util::read_input(i);
        println!("Actual: {}\n", run_all(input_str, i));
    }
}
