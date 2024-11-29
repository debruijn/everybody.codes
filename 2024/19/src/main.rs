use everybody_codes_util as util;
use everybody_codes_util::grid::{GridSparse2D, Grid, Point};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::zip;

type Pt = Point<isize, 2>;

fn get_diffs() -> Vec<Pt> {
    vec![
        Point([0, 1]),
        Point([1, 1]),
        Point([1, 0]),
        Point([1, -1]),
        Point([0, -1]),
        Point([-1, -1]),
        Point([-1, 0]),
        Point([-1, 1]),
    ]
}

// Function copied over from util::grid::Grid to adjust the shifts to rotate nicely.
// (Order in original method is: first along axis, then diagonal)
fn get_neighbors_options_grid(
    grid: &Grid<char>,
    pt: Point<isize, 2>,
) -> Vec<(Point<isize, 2>, char)> {
    let diffs = get_diffs();

    let mut res = Vec::new();
    for diff in diffs {
        let this = pt + diff;
        if grid.contains(this) {
            res.push((this, grid.get_pt(this)))
        }
    }
    res
}

// Function copied over from util::grid::GridSparse2D to adjust the shifts to rotate nicely.
// (Order in original method is: first along axis, then diagonal)
fn get_neighbors_options_gridsparse(grid: &GridSparse2D<Pt, isize>, pt: Pt) -> Vec<(Pt, Pt)> {
    let diffs = get_diffs();

    let mut res = Vec::new();
    for diff in diffs {
        let this = pt + diff;
        if grid.contains(this) {
            res.push((this, grid.get_pt(this)))
        }
    }
    res
}


fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    // Part 1: do the process once
    let mut key = input_str[0].trim().chars().cycle();
    let mut grid = Grid::from_string(input_str[2..].to_owned());

    for pt in grid.get_all_pts() {
        let neighbors: Vec<(Pt, char)> = get_neighbors_options_grid(&grid, pt);
        if neighbors.len() < 8 {
            continue;
        }
        let (mut locs, chars): (Vec<Pt>, Vec<char>) = neighbors.into_iter().unzip();
        if key.next().unwrap() == 'R' {  // rotate right -> locs to left
            locs.rotate_left(1);
        } else {
            locs.rotate_right(1);
        }
        for (loc, char_) in zip(&locs, chars) {
            grid.set_pt(char_, *loc);
        }
    }

    // Find solution inbetween > and <
    let pt_1 = grid.filter_first('>');
    let pt_2 = grid.filter_first('<');
    let mut res = Vec::new();
    for i in pt_1.0[1] + 1..pt_2.0[1] {
        res.push(grid.get((pt_1.0[0], i)));
    }
    format!("{}", res.iter().map(|x| x.to_string()).collect::<String>())
}

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    // Part 2: naively repeat the process 100 times
    let mut grid = Grid::from_string(input_str[2..].to_owned());
    for _ in 0..100 {
        let mut key = input_str[0].trim().chars().cycle();  // Don't forget to reset key
        for pt in grid.get_all_pts() {
            let neighbors: Vec<(Pt, char)> = get_neighbors_options_grid(&grid, pt);
            if neighbors.len() < 8 {
                continue;
            }
            let (mut locs, chars): (Vec<Pt>, Vec<char>) = neighbors.into_iter().unzip();
            if key.next().unwrap() == 'R' {
                // rotate right -> locs to left
                locs.rotate_left(1);
            } else {
                locs.rotate_right(1);
            }
            for (loc, char_) in zip(&locs, chars) {
                grid.set_pt(char_, *loc);
            }
        }
    }
    let pt_1 = grid.filter_first('>');
    let pt_2 = grid.filter_first('<');
    let mut res = Vec::new();
    let i = pt_1.0[0];
    for j in pt_1.0[1] + 1..pt_2.0[1] {
        res.push(grid.get((i, j)))
    }
    format!("{}", res.iter().map(|x| x.to_string()).collect::<String>())
}


fn single_iter_mapping<'a>(
    grid: &GridSparse2D<char, isize>,
    this_grid: &'a mut GridSparse2D<Point<isize, 2>, isize>,
    key: String,
) -> HashMap<&'a Pt, &'a Pt> {
    // For part 3: summarize the shift of one iteration of the process in one mapping of Pt -> Pt.
    let all_pts = grid.get_all_pts();
    for pt in all_pts.iter() {
        this_grid.set_pt(**pt, **pt);
    }
    let mut key = key.chars().cycle();

    for pt in grid.get_all_pts_sorted() {
        let neighbors: Vec<(Pt, Pt)> = get_neighbors_options_gridsparse(&this_grid, *pt);
        if neighbors.len() < 8 {
            continue;
        }
        let (mut locs, new_locs): (Vec<Pt>, Vec<Pt>) = neighbors.into_iter().unzip();
        if key.next().unwrap() == 'R' {
            // rotate right -> locs to left
            locs.rotate_left(1);
        } else {
            locs.rotate_right(1);
        }
        for (loc, loc_) in zip(&locs, new_locs) {
            this_grid.set_pt(loc_, *loc);
        }
    }

    let mapping = HashMap::from_iter(this_grid.get_data().into_iter().map(|x| (x.1, x.0)));
    mapping
}


fn double_iter_mapping<'a>(mapping: &HashMap<&'a Pt, &'a Pt>) -> HashMap<&'a Pt, &'a Pt> {
    // For part 3: apply mapping to itself to double the period that it maps
    let (keys_from, mut keys_to): (Vec<&Pt>, Vec<&Pt>) =
        mapping.iter().map(|x| (*x.0, *x.1)).unzip();
    keys_to = keys_to.into_iter().map(|x| mapping[x]).collect_vec();
    HashMap::from_iter(zip(keys_from, keys_to))
}

fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    let mut grid = GridSparse2D::from_string(input_str[2..].to_owned(), Vec::new());
    let mut target: isize = if _example { 100 } else { 1048576000 };

    // Construct mapping for 1 iteration of the process
    let mut pt_grid: GridSparse2D<Pt, isize> = GridSparse2D::new();
    let mut mapping = single_iter_mapping(&grid, &mut pt_grid, input_str[0].to_string());

    // Construct mapping for nr of process iterations equal to all powers of 2 up to target nr
    let mut pow: u32 = 1;
    let mut pow_mapping = Vec::new();
    pow_mapping.push(mapping.clone());
    while 2_isize.pow(pow) < target {
        mapping = double_iter_mapping(&mapping);
        pow_mapping.push(mapping.clone());
        pow += 1;
    }

    // Apply to all points the mapping of highest power of 2 below target, and repeat until target=0
    let (mut new_keys, values): (Vec<Pt>, Vec<char>) =
        grid.get_data().into_iter().map(|x| (*x.0, *x.1)).unzip();
    while target > 0 {
        let this_mapping = pow_mapping.pop().unwrap();
        if 2_isize.pow(pow_mapping.len() as u32) > target {
            continue;
        }
        new_keys = new_keys
            .into_iter()
            .map(|x| *this_mapping[&x])
            .collect_vec();
        target -= 2_isize.pow(pow_mapping.len() as u32);
    }

    // For reusing the same way to get the result between > and <: adjust grid to use the new keys
    let new_data = HashMap::from_iter(zip(new_keys, values));
    grid.set_data(new_data);

    // Find solution inbetween > and < like before
    let pt_1 = grid.filter_first('>');
    let pt_2 = grid.filter_first('<');
    let mut res = Vec::new();
    let i = pt_1.0[0];
    for j in pt_1.0[1] + 1..pt_2.0[1] {
        res.push(grid.get((i, j)));
    }
    let res = res.iter().map(|x| x.to_string()).collect::<String>();
    format!("{}", res)
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
    println!("Example: {}", util::run(run_part2, -2));
    println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Actual: {}", util::run(run_part3, -2));
    println!("Actual: {}\n", util::run(run_part3, 3));
}
