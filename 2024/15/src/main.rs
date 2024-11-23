use everybody_codes_util as util;
use everybody_codes_util::grid::{Grid, Point};
use itertools::Itertools;
use std::collections::{HashSet};

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    let grid: Grid<char> = Grid::from_string(input_str.clone());
    let loc = Point([0, input_str[0].len() as isize / 2]);
    let mut seen = HashSet::new();
    let mut queue: Vec<(Point<isize, 2>, isize)> = Vec::new();
    let final_steps;
    queue.push((loc, 0));
    loop {
        let (this_pt, this_steps) = queue.remove(0);
        if grid.get_pt(this_pt) == 'H' {
            final_steps = this_steps * 2;
            break;
        }

        let mut neighbors: Vec<(Point<isize, 2>, char)> = grid.get_neighbors_ok(this_pt);

        neighbors = neighbors
            .into_iter()
            .filter(|x| x.1 != '#')
            .filter(|x| !seen.contains(&x.0))
            .collect_vec();
        for neighbor in neighbors.iter() {
            seen.insert(neighbor.0);
            queue.push((neighbor.0, this_steps + 1))
        }
    }

    final_steps.to_string()
}

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let grid: Grid<char> = Grid::from_string(input_str.clone());
    let herb_types = input_str
        .join("")
        .chars()
        .filter(|x| *x != '.' && *x != '#' && *x != '~')
        .unique()
        .collect_vec();
    let start_loc = Point([0, input_str[0].len() as isize / 2]);
    let mut seen: HashSet<(Point<isize, 2>, Vec<bool>)> = HashSet::new();
    let mut queue: Vec<(Point<isize, 2>, isize, Vec<bool>)> = Vec::new();
    let final_steps;
    let mut start_state = Vec::new();
    for _ in 0..herb_types.len() {
        start_state.push(false)
    }
    queue.push((start_loc, 0, start_state));
    loop {
        let (this_pt, this_steps, this_state) = queue.remove(0);
        if this_pt == start_loc && this_state.iter().all(|x| *x) {
            final_steps = this_steps;
            break;
        }

        let mut neighbors: Vec<(Point<isize, 2>, char)> = grid.get_neighbors_ok(this_pt);

        neighbors = neighbors
            .into_iter()
            .filter(|x| x.1 != '#' && x.1 != '~')
            .filter(|x| !seen.contains(&(x.0, this_state.clone())))
            .collect_vec();
        for neighbor in neighbors.iter() {
            seen.insert((neighbor.0, this_state.clone()));
            let mut new_state = this_state.clone();
            let this_char = grid.get_pt(neighbor.0);
            if herb_types.contains(&this_char) {
                let herb_loc = herb_types
                    .iter()
                    .position(|x| x == &grid.get_pt(neighbor.0))
                    .unwrap();
                if !new_state[herb_loc] {
                    new_state[herb_loc] = true;
                    seen.insert((neighbor.0, new_state.clone()));
                }
            }
            queue.push((neighbor.0, this_steps + 1, new_state))
        }
    }
    final_steps.to_string()
}


fn run_algo(grid: &Grid<char>, start_loc: Point<isize,2>, herb_types: Vec<&char>, target_loc: Point<isize,2>) -> isize {
    let mut seen: HashSet<(Point<isize, 2>, Vec<bool>)> = HashSet::new();
    let mut queue: Vec<(Point<isize, 2>, isize, Vec<bool>)> = Vec::new();
    let final_steps;
    let mut start_state = Vec::new();
    for _ in 0..herb_types.len() {
        start_state.push(false)
    }
    queue.push((start_loc, 0, start_state));
    loop {
        let (this_pt, this_steps, this_state) = queue.remove(0);
        if this_pt == target_loc && this_state.iter().all(|x| *x) {
            final_steps = this_steps;
            break;
        }

        let mut neighbors: Vec<(Point<isize, 2>, char)> = grid.get_neighbors_ok(this_pt);

        neighbors = neighbors
            .into_iter()
            .filter(|x| x.1 != '#' && x.1 != '~')
            .filter(|x| !seen.contains(&(x.0, this_state.clone())))
            .collect_vec();
        for neighbor in neighbors.iter() {
            seen.insert((neighbor.0, this_state.clone()));
            let mut new_state = this_state.clone();
            let this_char = grid.get_pt(neighbor.0);
            if herb_types.contains(&&this_char) {
                let herb_loc = herb_types
                    .iter()
                    .position(|x| *x == &grid.get_pt(neighbor.0))
                    .unwrap();
                if !new_state[herb_loc] {
                    new_state[herb_loc] = true;
                    seen.insert((neighbor.0, new_state.clone()));
                }
            }
            queue.push((neighbor.0, this_steps + 1, new_state))
        }
    }
    println!("{}", final_steps);
    final_steps
}


fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    let mut grid: Grid<char> = Grid::from_string(input_str.clone());
    let loc_E = grid.filter_key('E').into_iter().last().unwrap();
    let loc_R = grid.filter_key('R').into_iter().next().unwrap();

    let herb_types = input_str
        .join("")
        .chars()
        .filter(|x| *x != '.' && *x != '#' && *x != '~')
        .unique()
        .collect_vec();
    let herb_E = herb_types.iter().filter(|x| grid.filter_key(**x).iter().filter(|y| y.0[1] < loc_E.0[1]).count()>0).filter(|x| **x != 'E').collect_vec();
    let herb_R = herb_types.iter().filter(|x| grid.filter_key(**x).iter().filter(|y| y.0[1] > loc_R.0[1]).count()>0).filter(|x| **x != 'R').collect_vec();
    let herb_S = herb_types.iter().filter(|x| grid.filter_key(**x).iter().filter(|y| (y.0[1] >= loc_E.0[1]) && (y.0[1] <= loc_R.0[1])).count()>0).collect_vec();
    let start_loc = Point([0, input_str[0].len() as isize / 2]);

    grid.set_pt('#', loc_E + Point([-2, 0]));
    grid.set_pt('#', loc_R + Point([-2, 0]));
    let center_grid = run_algo(&grid, start_loc, herb_S, start_loc);

    grid.set_pt('.', loc_E + Point([-2, 0]));
    grid.set_pt('.', loc_R + Point([-2, 0]));
    grid.set_pt('#', loc_E + Point([0, 2]));
    grid.set_pt('#', loc_R + Point([0, -2]));
    let left_grid = run_algo(&grid, loc_E, herb_E, loc_E);
    let right_grid = run_algo(&grid, loc_R, herb_R, loc_R);

    (center_grid+left_grid+right_grid).to_string()
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    util::run(run_part1, -1); // Included because first call is slower for allocation reasons
    util::run(run_part1, 1);
    println!("Example: {}", util::run(run_part1, -1));
    println!("Actual: {}\n", util::run(run_part1, 1));

    // Part 2: example and actual
    println!("Part 2");
    println!("Example: {}", util::run(run_part2, 1));
    println!("Example: {}", util::run(run_part2, -2));
    println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Actual: {}\n", util::run(run_part3, 3));
}
