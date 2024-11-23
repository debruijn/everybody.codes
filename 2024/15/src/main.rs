use everybody_codes_util as util;
use everybody_codes_util::grid::{Grid, GridSparse2D, Point};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

type Pt = Point<isize, 2>;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    let grid: GridSparse2D<u8, isize> = GridSparse2D::from_string(input_str.clone(), vec![b'#']);
    let loc = Point([0, input_str[0].len() as isize / 2]);
    let mut seen = HashSet::new();
    let mut queue: Vec<(Pt, isize)> = Vec::new();
    let final_steps;
    queue.push((loc, 0));
    loop {
        let (this_pt, this_steps) = queue.remove(0);
        if grid.get_pt(this_pt) == b'H' {
            final_steps = this_steps * 2;
            break;
        }

        let mut neighbors: Vec<(Pt, u8)> = grid.get_neighbors_ok(this_pt);

        neighbors = neighbors
            .into_iter()
            .filter(|x| x.1 != b'#')
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
    run_algo(&grid, start_loc, herb_types).to_string()
}

fn run_algo(grid: &Grid<char>, start_loc: Pt, herb_types: Vec<char>) -> isize {
    // Run BFS with state equal to number steps and which herbs you have collected
    // Trim on seen locs with the same state (ign steps since that will then always be better)
    // Stop if back at start_point with all herbs
    let mut seen: HashSet<(Pt, Vec<bool>)> = HashSet::new();
    let mut queue: VecDeque<(Pt, isize, Vec<bool>)> = VecDeque::new();
    let final_steps; // Preallocate final result
    let start_state = vec![false; herb_types.len()];
    queue.push_back((start_loc, 0, start_state));
    loop {
        let (this_pt, this_steps, this_state) = queue.pop_front().unwrap();
        if this_pt == start_loc && this_state.iter().all(|x| *x) {
            final_steps = this_steps;
            break;
        }

        // Get neighbors and filter on walls/lake and what we have seen already
        let mut neighbors: Vec<(Pt, char)> = grid.get_neighbors_ok(this_pt);
        neighbors = neighbors
            .into_iter()
            .filter(|x| x.1 != '#' && x.1 != '~')
            .filter(|x| !seen.contains(&(x.0, this_state.clone())))
            .collect_vec();

        // All neighbors left over will be added to queue (and seen). Determine with what state.
        for neighbor in neighbors.iter() {
            seen.insert((neighbor.0, this_state.clone()));
            let mut new_state = this_state.clone();
            let this_char = grid.get_pt(neighbor.0);
            if herb_types.contains(&&this_char) {
                let herb_loc = herb_types
                    .iter()
                    .position(|x| x == &grid.get_pt(neighbor.0))
                    .unwrap();
                if !new_state[herb_loc] {
                    new_state[herb_loc] = true;
                    seen.insert((neighbor.0, new_state.clone()));
                }
            }
            queue.push_back((neighbor.0, this_steps + 1, new_state))
        }
    }
    final_steps
}

fn filter_herbs(herb_types: &Vec<char>, pt: Pt, left: bool, grid: &Grid<char>) -> Vec<char> {
    // Filter the herbs to all herbs to the left or right of the given point.
    let this_herbs = herb_types
        .iter()
        .filter(|x| **x != grid.get_pt(pt))
        .collect_vec();
    let mut new_herbs = Vec::new();
    for herb in this_herbs.iter() {
        if grid
            .filter_key(**herb)
            .iter()
            .filter(|x| {
                if left {
                    x.0[1] < pt.0[1]
                } else {
                    x.0[1] > pt.0[1]
                }
            })
            .count()
            > 0
        {
            new_herbs.push(**herb)
        }
    }
    new_herbs
}

fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    // Split grid into 3 based on last E and first R locations (could have used both Ks as well)
    // These are independent subgrids, so you can solve left and right with E and R as start point,
    // solve middle with E and R as normal herbs, and add the three together.

    // Get grid and locs
    let mut grid: Grid<char> = Grid::from_string(input_str.clone());
    let loc_e = grid.filter_last('E');
    let loc_r = grid.filter_first('R');
    let start_loc = Point([0, input_str[0].len() as isize / 2]);

    // Get herbs to collect in each subgrids
    let herb_types = input_str
        .join("")
        .chars()
        .filter(|x| *x != '.' && *x != '#' && *x != '~')
        .unique()
        .collect_vec();
    let herb_e = filter_herbs(&herb_types, loc_e, true, &grid);
    let herb_r = filter_herbs(&herb_types, loc_r, false, &grid);
    let herb_s = herb_types
        .into_iter()
        .filter(|x| !herb_e.contains(x) && !herb_r.contains(x))
        .collect_vec();

    // For middle grid: block points off to avoid diversions in the other subgrids (also works without but slower)
    grid.set_pt('#', loc_e + Point([-2, 0]));
    grid.set_pt('#', loc_r + Point([-2, 0]));
    let center_grid = run_algo(&grid, start_loc, herb_s);

    // For left/right grids: revert the above blockage, and add two new ones.
    grid.set_pt('.', loc_e + Point([-2, 0]));
    grid.set_pt('.', loc_r + Point([-2, 0]));
    grid.set_pt('#', loc_e + Point([0, 2]));
    grid.set_pt('#', loc_r + Point([0, -2]));
    let left_grid = run_algo(&grid, loc_e, herb_e);
    let right_grid = run_algo(&grid, loc_r, herb_r);

    (center_grid + left_grid + right_grid).to_string()
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
    println!("Example 1: {}", util::run(run_part2, -1));
    println!("Example 2: {}", util::run(run_part2, -2));
    println!("Actual 1: {}", util::run(run_part2, 1));
    println!("Actual 2: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Actual: {}\n", util::run(run_part3, 3));
}
