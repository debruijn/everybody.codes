use everybody_codes_util as util;
use everybody_codes_util::grid::Point;
use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};

type Pt = Point<isize, 3>;
type PtSet = HashSet<Pt>;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    let mut pt: Pt = Pt::new([0, 0, 0]);

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

fn construct_all_branches(input_str: Vec<String>) -> (PtSet, PtSet) {
    let mut leafs = PtSet::new();
    let mut hist = PtSet::new();
    for row in input_str.iter() {
        let mut pt: Pt = Pt::new([0, 0, 0]);
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

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let (hist, _) = construct_all_branches(input_str);
    hist.len().to_string()
}

pub fn get_neighbors3d(pt: Pt) -> [Pt; 6] {
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

fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    // Construct all branches and final leafs
    let (branches, leafs) = construct_all_branches(input_str);

    // Find min/max of leaf y-values, since the optimum is >99% certain to be in between
    // (only used if trim_greedily is set to true)
    let trim_greedily = false;
    let min_max = leafs.iter().map(|x| x.0[1]).minmax();
    let min_max = util::minmax(min_max);

    // Main algorithm: find all distances from each leaf to the trunk at all heights
    // -> Use mostly standard BFS & keep track of visited locs
    // -> Don't stop after first trunk reach to allow for multiple routes to different heights
    // -> Ignore new locs outside branches.
    // -> When on the trunk, ignore new locs outside trunk (if trim_greedily; not needed)
    // -> Ignore new locations on the trunk outside the min_max range (if trim_greedily, not needed)

    // Store results in res: for each point a map of trunk height to dist
    let mut res = Vec::new();
    for leaf in leafs {
        let mut queue = vec![(leaf, 0)];
        let mut leaf_hist = HashSet::new();
        leaf_hist.insert(leaf);
        let mut res_leaf: HashMap<isize, isize> = HashMap::new();
        while queue.len() > 0 {
            let (this_pt, this_dist) = *queue.iter().next().unwrap();
            queue.remove(0);
            let neighbors = if this_pt.0[0] == 0 && this_pt.0[2] == 0 {
                res_leaf.insert(this_pt.0[1], this_dist);
                vec![this_pt + Pt::new([0, 1, 0]), this_pt + Pt::new([0, -1, 0])]
            } else {
                get_neighbors3d(this_pt).into_iter().collect_vec()
            };
            let mut neighbors = neighbors
                .into_iter()
                .filter(|x| !leaf_hist.contains(x))
                .filter(|x| branches.contains(x))
                .collect_vec();
            if trim_greedily {
                neighbors = neighbors
                    .into_iter()
                    .filter(|x| {
                        x.0[0] != 0 || x.0[2] != 0 || (x.0[1] >= min_max[0] && x.0[1] <= min_max[1])
                    })
                    .collect_vec();
            }
            for neighbor in neighbors.iter() {
                queue.push((*neighbor, this_dist + 1));
                leaf_hist.insert(*neighbor);
            }
        }
        res.push(res_leaf);
    }

    // Process results: for all values of trunk that are reached, find the distance to that
    // height for each leaf, sum those for each trunk height, and take the minimum of that sum.
    let all_trunk_hs = res[0].keys().collect_vec();
    let final_res: isize = all_trunk_hs
        .iter()
        .map(|h| res.iter().map(|x| x[h]).sum::<isize>())
        .min()
        .unwrap();
    final_res.to_string()
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    println!("Example: {}", util::run(&run_part1, -1));
    println!("Actual: {}\n", util::run(&run_part1, 1));

    // Part 2: example and actual
    println!("Part 2");
    println!("Example: {}", util::run(&run_part2, -2));
    println!("Actual: {}\n", util::run(&run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example 1: {}", util::run(&run_part3, -3));
    println!("Example 2: {}", util::run(&run_part3, -4));
    println!("Actual: {}\n", util::run(&run_part3, 3));
}
