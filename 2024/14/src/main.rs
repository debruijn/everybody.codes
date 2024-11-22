use everybody_codes_util as util;
use everybody_codes_util::grid::Point;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

fn run_part1(input_str: Vec<String>) -> String {
    let mut pt: Point<isize, 3> = Point::new([0, 0, 0]);

    let mut max_height = 0;
    for step in input_str[0].split(',') {
        let this_char = step.chars().next().unwrap();
        pt = pt
            + match this_char {
                'U' => Point([0, step[1..].parse::<isize>().unwrap(), 0]),
                'D' => Point([0, -step[1..].parse::<isize>().unwrap(), 0]),
                'R' => Point([step[1..].parse::<isize>().unwrap(), 0, 0]),
                'L' => Point([-step[1..].parse::<isize>().unwrap(), 0, 0]),
                'F' => Point([0, 0, step[1..].parse::<isize>().unwrap()]),
                'B' => Point([0, 0, -step[1..].parse::<isize>().unwrap()]),
                _ => Point([0, 0, 0]),
            };
        max_height = max(max_height, pt.0[1])
    }
    max_height.to_string()
}

fn construct_all_branches(
    input_str: Vec<String>,
) -> (HashSet<Point<isize, 3>>, HashSet<Point<isize, 3>>) {
    let mut leafs = HashSet::new();
    let mut hist = HashSet::new();
    for row in input_str.iter() {
        let mut pt: Point<isize, 3> = Point::new([0, 0, 0]);
        for step in row.split(',') {
            let this_char = step.chars().next().unwrap();
            for _ in 0..step[1..].parse::<isize>().unwrap() {
                pt = pt
                    + match this_char {
                        'U' => Point([0, 1, 0]),
                        'D' => Point([0, -1, 0]),
                        'R' => Point([1, 0, 0]),
                        'L' => Point([-1, 0, 0]),
                        'F' => Point([0, 0, 1]),
                        'B' => Point([0, 0, -1]),
                        _ => Point([0, 0, 0]),
                    };
                hist.insert(pt);
            }
        }
        leafs.insert(pt);
    }
    (hist, leafs)
}

fn run_part2(input_str: Vec<String>) -> String {
    let (hist, _) = construct_all_branches(input_str);
    hist.len().to_string()
}

pub fn get_neighbors3d(pt: Point<isize, 3>) -> [Point<isize, 3>; 6] {
    let diffs = [
        Point([0, 1, 0]),
        Point([1, 0, 0]),
        Point([0, -1, 0]),
        Point([-1, 0, 0]),
        Point([0, 0, 1]),
        Point([0, 0, -1]),
    ];
    diffs
        .iter()
        .map(|x| pt + *x)
        .collect_vec()
        .try_into()
        .unwrap()
}

fn run_part3(input_str: Vec<String>) -> String {
    // Construct all branches and final leafs
    let (branches, leafs) = construct_all_branches(input_str);

    // Find min/max of leaf y-values, since the optimum is >99% certain to be in between
    let min_max = leafs.iter().map(|x| x.0[1]).minmax();
    let min_max = util::minmax(min_max);

    // First main algorithm: find all distances from each leaf to the trunk at all heights
    // -> Use mostly standard BFS & keep track of visited locs
    // -> Don't stop after first trunk reach to allow for multiple routes to different heights
    // -> Ignore new locs outside branches.
    // -> When on the trunk, ignore new locs outside trunk  # TODO is that needed?
    // -> Ignore new locations on the trunk outside the min_max range  # TODO is that needed?

    // Store results in res: map each point to vector of (height, dist)  # TODO can refactor
    let mut res: HashMap<Point<isize, 3>, Vec<(isize, isize)>> = HashMap::new();
    for leaf in leafs {
        let mut queue = vec![(leaf, 0)];
        let mut leaf_hist = HashSet::new();
        leaf_hist.insert(leaf);
        let mut res_leaf = Vec::new();
        while queue.len() > 0 {
            let (this_pt, this_dist) = *queue.iter().next().unwrap();
            queue.remove(0);
            let neighbors = if this_pt.0[0] == 0 && this_pt.0[2] == 0 {
                res_leaf.push((this_pt.0[1], this_dist));
                vec![
                    this_pt + Point::new([0, 1, 0]),
                    this_pt + Point::new([0, -1, 0]),
                ]
            } else {
                get_neighbors3d(this_pt).into_iter().collect_vec()
            };
            let neighbors = neighbors
                .iter()
                .filter(|x| !leaf_hist.contains(x))
                .filter(|x| branches.contains(x))
                .filter(|x| {
                    x.0[0] != 0 || x.0[2] != 0 || (x.0[1] >= min_max[0] && x.0[1] <= min_max[1])
                })
                .collect_vec();
            for neighbor in neighbors.iter() {
                queue.push((**neighbor, this_dist + 1));
                leaf_hist.insert(**neighbor);
            }
        }
        res.insert(leaf, res_leaf);
    }

    // Second main algorithm: for all values of trunk that are reached, find the distance to that
    // height for each leaf, sum those for each trunk height, and take the minimum of that sum.
    let all_trunk_hs = res
        .values()
        .map(|x| x.iter().map(|y| y.0))
        .flatten()
        .collect_vec();
    let final_res: isize = all_trunk_hs
        .iter()
        .map(|h| {
            res.values()
                .map(|x| x.iter().filter(|y| y.0 == *h).map(|y| y.1).sum::<isize>())
                .sum::<isize>()
        })
        .min()
        .unwrap();
    final_res.to_string()
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
    println!("Example: {}", run_part2(input_str));
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example 1: {}", run_part3(input_str));
    let input_str = util::read_input(-4);
    println!("Example 2: {}", run_part3(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));
}
