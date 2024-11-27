use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use everybody_codes_util as util;
use util::grid::GridSparse2D;
use util::grid::Point;

type Pt = Point<isize,2>;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    let grid = GridSparse2D::from_string(input_str, vec!(b'#'));
    let trees = grid.filter_key(b'P');
    let start = Pt::new([1,0]);
    let mut count = 0;
    let mut queue = VecDeque::from(vec!((start, 0)));
    let mut hist = Vec::new();
    hist.push(start);
    let mut final_step_count= 0;
    while count < trees.len() {
        let (this_loc, this_count) = queue.pop_front().unwrap();


        let mut neighbors = grid.get_neighbors_ok(this_loc);
        neighbors = neighbors.into_iter().filter(|x| !hist.contains(&x.0)).collect_vec();

        for neighbor in neighbors.iter() {
            if neighbor.1 == b'P' {
                count += 1;
                if count == trees.len() {
                    final_step_count = this_count + 1;
                }
            }

            hist.push(neighbor.0);
            queue.push_back((neighbor.0, this_count+1))

        }

    }

    final_step_count.to_string()
}

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let grid = GridSparse2D::from_string(input_str, vec!(b'#'));
    let trees = grid.filter_key(b'P');
    let start1 = Pt::new([1,0]);
    let start2 = Pt::new([grid.get_dims()[0], grid.get_dims()[1]-1]);
    let mut count = 0;
    let mut queue = VecDeque::from(vec!((start1, 0), (start2, 0)));
    let mut hist = Vec::new();
    hist.push(start1);
    hist.push(start2);
    let mut final_step_count= 0;

    println!("{:?}, {:?}", start2, grid.get_dims());

    while count < trees.len() {
        let (this_loc, this_count) = queue.pop_front().unwrap();


        let mut neighbors = grid.get_neighbors_ok(this_loc);
        neighbors = neighbors.into_iter().filter(|x| !hist.contains(&x.0)).collect_vec();

        for neighbor in neighbors.iter() {
            if neighbor.1 == b'P' {
                if !hist.contains(&neighbor.0) {
                    count += 1;
                    if count == trees.len() {
                        final_step_count = this_count + 1;
                    }
                }
            }

            hist.push(neighbor.0);
            queue.push_back((neighbor.0, this_count+1))

        }

    }

    final_step_count.to_string()
}


// fn run_algo(grid: &GridSparse2D<u8, isize>, start: &Pt) -> isize {
//     let trees = grid.filter_key(b'P');
//     let mut count = 0;
//     let mut total_time = 0;
//     let mut queue = VecDeque::from(vec!((*start, 0)));
//     let mut hist = Vec::new();
//     hist.push(*start);
//     while count < trees.len() {
//         let (this_loc, this_count) = queue.pop_front().unwrap();
//
//
//         let mut neighbors = grid.get_neighbors_ok(this_loc);
//         neighbors = neighbors.into_iter().filter(|x| !hist.contains(&x.0)).collect_vec();
//
//         for neighbor in neighbors.iter() {
//             if neighbor.1 == b'P' {
//                 // println!("{}, {}, {:?}", this_count+1, count, neighbor);
//                 count += 1;
//                 total_time += this_count + 1;
//             }
//
//             hist.push(neighbor.0);
//             queue.push_back((neighbor.0, this_count+1))
//
//         }
//
//     }
//     total_time
// }
//
//
// fn run_part3(input_str: Vec<String>, _example: bool) -> String {
//     let mut grid = GridSparse2D::from_string(input_str, vec!(b'#'));
//     let pts = grid.filter_key(b'.');
//     let mut res = Vec::new();
//     println!("{}", pts.iter().count());
//     for (i, pt) in pts.iter().enumerate() {
//         let this = run_algo(&grid, pt);
//         res.push(this);
//         println!("{}, {}, {:?}, {}", i, this, pt, res.iter().min().unwrap());
//     }
//     res.iter().min().unwrap().to_string()
// }


fn run_algo_fast(grid: &GridSparse2D<u8, isize>, start: &Pt) -> HashMap<Pt, isize> {
    let trees = grid.filter_key(b'.');
    let mut count = 0;
    let mut queue = VecDeque::from(vec!((*start, 0)));
    let mut dist: HashMap<Pt, isize> = HashMap::new();
    let mut hist = Vec::new();
    hist.push(*start);
    while count < trees.len() {
        let (this_loc, this_count) = queue.pop_front().unwrap();


        let mut neighbors = grid.get_neighbors_ok(this_loc);
        neighbors = neighbors.into_iter().filter(|x| !hist.contains(&x.0)).collect_vec();

        for neighbor in neighbors.iter() {
            if neighbor.1 == b'.' {
                dist.insert(neighbor.0, this_count+1);
                count += 1;
            }

            hist.push(neighbor.0);
            queue.push_back((neighbor.0, this_count+1))

        }

    }
    dist
}


fn run_part3_fast(input_str: Vec<String>, _example: bool) -> String {
    let grid = GridSparse2D::from_string(input_str, vec!(b'#'));
    let trees = grid.filter_key(b'P');


    // Run algorithm starting from each tree, mapping distance to all pts
    // Then find point with lowest sum of distances;
    let mut tree_map = HashMap::new();
    for tree in trees.iter() {
        tree_map.insert(tree, run_algo_fast(&grid, tree));
    }

    let mut min_pt = isize::MAX;
    for pt in tree_map[&trees[0]].keys() {
        let this_dist = tree_map.values().map(|x| x[pt]).sum::<isize>();
        min_pt = min(min_pt, this_dist);
    }

    min_pt.to_string()
}


fn main() {
    // Part 1: example and actual
    println!("Part 1");
    util::run(run_part1, -1);  // Included because first call is slower for allocation reasons
    util::run(run_part1, 1);
    println!("Example: {}", util::run(run_part1, -1));
    println!("Actual: {}\n", util::run(run_part1, 1));

    // Part 2: example and actual
    println!("Part 2");
    println!("Example: {}", util::run(run_part2, -2));
    println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example: {}", util::run(run_part3_fast, -3));
    println!("Actual: {}\n", util::run(run_part3_fast, 3));

}
