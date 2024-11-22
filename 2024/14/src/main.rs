use everybody_codes_util as util;
use everybody_codes_util::grid::Point;
use itertools::{Itertools, MinMaxResult};
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};

fn run_part1(input_str: Vec<String>) -> String {
    let mut pt: Point<isize, 3> = Point::new([0, 0, 0]);

    let mut max_height = 0;
    for row in input_str[0].split(',') {
        if row.chars().next().unwrap() == 'U' {
            pt = pt + Point([0, row[1..].parse::<isize>().unwrap(), 0]);
        }
        if row.chars().next().unwrap() == 'D' {
            pt = pt + Point([0, -row[1..].parse::<isize>().unwrap(), 0]);
        }
        if row.chars().next().unwrap() == 'R' {
            pt = pt + Point([row[1..].parse::<isize>().unwrap(), 0, 0]);
        }
        if row.chars().next().unwrap() == 'L' {
            pt = pt + Point([-row[1..].parse::<isize>().unwrap(), 0, 0]);
        }
        if row.chars().next().unwrap() == 'F' {
            pt = pt + Point([0, 0, row[1..].parse::<isize>().unwrap()]);
        }
        if row.chars().next().unwrap() == 'B' {
            pt = pt + Point([0, 0, -row[1..].parse::<isize>().unwrap()]);
        }
        max_height = max(max_height, pt.0[1])
    }
    max_height.to_string()
}

fn run_part2(input_str: Vec<String>) -> String {
    // let mut pt: Point<isize, 3> = Point::new([0, 0, 0]);

    let mut hist = HashSet::new();

    for row in input_str.iter() {
        // println!("{row}");
        let mut pt: Point<isize, 3> = Point::new([0, 0, 0]);

        for step in row.split(',') {
            let this_char = step.chars().next().unwrap();
            for _ in 0..step[1..].parse::<isize>().unwrap() {
                if this_char == 'U' {
                    pt = pt + Point([0, 1, 0]);
                }
                if this_char == 'D' {
                    pt = pt + Point([0, -1, 0]);
                }
                if this_char == 'R' {
                    pt = pt + Point([1, 0, 0]);
                }
                if this_char == 'L' {
                    pt = pt + Point([-1, 0, 0]);
                }
                if this_char == 'F' {
                    pt = pt + Point([0, 0, 1]);
                }
                if this_char == 'B' {
                    pt = pt + Point([0, 0, -1]);
                }
                hist.insert(pt);
                // println!("{:?}", hist)
            }
        }
    }
    // println!("{:?}", hist);
    hist.len().to_string()
}

pub fn get_neighbors3(pt: Point<isize, 3>) -> [Point<isize, 3>; 6] {
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
    let mut leafs = HashSet::new();
    let mut hist = HashSet::new();

    for row in input_str.iter() {
        // println!("{row}");
        let mut pt: Point<isize, 3> = Point::new([0, 0, 0]);

        for step in row.split(',') {
            let this_char = step.chars().next().unwrap();
            for _ in 0..step[1..].parse::<isize>().unwrap() {
                if this_char == 'U' {
                    pt = pt + Point([0, 1, 0]);
                }
                if this_char == 'D' {
                    pt = pt + Point([0, -1, 0]);
                }
                if this_char == 'R' {
                    pt = pt + Point([1, 0, 0]);
                }
                if this_char == 'L' {
                    pt = pt + Point([-1, 0, 0]);
                }
                if this_char == 'F' {
                    pt = pt + Point([0, 0, 1]);
                }
                if this_char == 'B' {
                    pt = pt + Point([0, 0, -1]);
                }
                hist.insert(pt);
                // println!("{:?}", hist)
            }
        }
        leafs.insert(pt);
    }
    let min_max = leafs.iter().map(|x| x.0[1]).minmax();
    let min_max = match min_max {
        MinMaxResult::NoElements => [0, 0],
        MinMaxResult::OneElement(a) => [a, a],
        MinMaxResult::MinMax(a, b) => [max(a, 0), b],
    };

    // println!("{:?}, {:?}, {:?}", leafs, min_max, hist);

    // For each leaf:
    // - Step one away until trunk in area -> store
    // - New candidates when on trunk: only on trunk
    // - Get distances to all values within range

    let mut res: HashMap<Point<isize, 3>, Vec<(isize, isize)>> = HashMap::new();
    for leaf in leafs {
        let mut queue = vec![(leaf, 0)];
        let mut leaf_hist = HashSet::new();
        leaf_hist.insert(leaf);
        let mut res_hist = Vec::new();
        while queue.len() > 0 {
            // for _ in 0..10 {
            let (this_pt, this_dist) = *queue.iter().next().unwrap();
            queue.remove(0);
            let mut neighbors = if this_pt.0[0] == 0 && this_pt.0[2] == 0 {
                res_hist.push((this_pt.0[1], this_dist));
                vec![
                    this_pt + Point::new([0, 1, 0]),
                    this_pt + Point::new([0, -1, 0]),
                ]
            } else {
                get_neighbors3(this_pt).into_iter().collect_vec()
            };
            let neighbors = neighbors
                .iter()
                .filter(|x| !leaf_hist.contains(x))
                .filter(|x| hist.contains(x))
                .filter(|x| {
                    x.0[0] != 0 || x.0[2] != 0 || (x.0[1] >= min_max[0] && x.0[1] <= min_max[1])
                })
                .collect_vec();
            for neighbor in neighbors.iter() {
                queue.push((**neighbor, this_dist + 1));
                leaf_hist.insert(**neighbor);
            }
            // println!("{}, {:?}, {:?}", queue.len(), queue, res_hist)
        }
        res.insert(leaf, res_hist);
    }

    let all_trunk_hs = res.values().map(|x| x.iter().map(|y| y.0)).flatten().collect_vec();

    let final_res: isize = all_trunk_hs.iter()
        .map(|h| {
            res.values()
                .map(|x| x.iter().filter(|y| y.0 == *h).map(|y| y.1).sum::<isize>())
                .sum::<isize>()
        })
        .min()
        .unwrap();

    // let mut min_val = 100000;
    // for try_val in min_max[0]..=min_max[1] {
    //     let try_pt = Point::new([0, try_val, 0]);
    //
    //
    //
    //     // for leaf in leafs.iter() {
    //     //     let this = leaf.manhattan_dist(try_pt);
    //     //     // println!("{:?}, {:?}, {}", try_pt, leaf, this)
    //     // }
    //     let this_val = leafs.iter().map(|x| x.manhattan_dist(try_pt)).sum();
    //     println!("{try_val}, {this_val}");
    //     min_val = min(min_val, this_val);
    // }

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
