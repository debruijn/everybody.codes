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


fn run_bfs_pt3(grid: &GridSparse2D<u8, isize>, start: &Pt) -> HashMap<Pt, isize> {
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


fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    let grid = GridSparse2D::from_string(input_str, vec!(b'#'));
    let trees = grid.filter_key(b'P');


    // Run algorithm starting from each tree, mapping distance to all pts
    // Then find point with lowest sum of distances;
    let mut tree_map = HashMap::new();
    for tree in trees.iter() {
        tree_map.insert(tree, run_bfs_pt3(&grid, tree));
    }

    let mut min_pt = isize::MAX;
    for pt in tree_map[&trees[0]].keys() {
        let this_dist = tree_map.values().map(|x| x[pt]).sum::<isize>();
        min_pt = min(min_pt, this_dist);
    }

    min_pt.to_string()
}



fn run_part3_faster(input_str: Vec<String>, _example: bool) -> String {
    // Construct grid, and get points for trees and allowed points for the path
    let grid = GridSparse2D::from_string(input_str, vec!(b'#'));
    let trees = grid.filter_key(b'P');
    let allowed = grid.filter_keys(vec!(b'.', b'P'));

    // Construct queue (what to do next), hist (what has been done before) and dist (target metric)
    let mut queue: VecDeque<(usize, Pt, usize)> = VecDeque::from(trees.iter().enumerate().map(|x| (x.0, *x.1, 0)).collect_vec());
    let mut dist: HashMap<Pt, usize> = HashMap::from_iter(allowed.iter().map(|x| (*x, 0)));
    let mut hist = Vec::new();
    for tree in trees.iter() {
        hist.push(vec!(*tree))
    }

    // Helper variables: best metric so far and how many trees have reached each point
    let mut best = usize::MAX;
    let mut count_trees: HashMap<Pt, usize> = HashMap::from_iter(allowed.into_iter().map(|x| (x, 0)));

    // Run BFS until queue is empty.
    // - Trimming: if there is a candidate solution, ignore all points that can't reach that
    // - This is not guaranteed and with harsher trimming (see below) you don't get an optimum.
    while queue.len() > 0 {
        let (this_tree, this_loc, this_count) = queue.pop_front().unwrap();

        // Neighbors of the current location. Filter points that have been seen in regard to tree.
        let mut neighbors = grid.get_neighbors_ok(this_loc);
        neighbors = neighbors.into_iter().filter(|x| !hist[this_tree].contains(&x.0)).collect_vec();

        for neighbor in neighbors.iter() {
            // For each remaining neighbor that is not a tree, update dist and count.
            if neighbor.1 == b'.' {
                count_trees.insert(neighbor.0, count_trees[&neighbor.0] + 1);
                dist.insert(neighbor.0, dist[&neighbor.0] + this_count+1);
                if count_trees[&neighbor.0] == trees.len() {
                    // If this neighbor has now been seen by all trees, check if it is new best
                    if dist[&neighbor.0] < best {
                        best = dist[&neighbor.0];
                    }
                }
            }
            hist[this_tree].push(neighbor.0);

            // Doesn't work! Even though point will not be solution, it might be on the path towards
            // the solution for one tree.
            // let best_achievable = dist[&neighbor.0] + this_count * (trees.len() - count_trees[&neighbor.0]);

            // Instead, a conservative fallback.
            let best_achievable = dist[&neighbor.0];
            if best_achievable <= best {
                queue.push_back((this_tree, neighbor.0, this_count + 1))
            }

        }

    }
    best.to_string()
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
    println!("Example initial solution: {}", util::run(run_part3, -3));
    println!("Example faster solution (for bigger data): {}", util::run(run_part3_faster, -3));
    println!("Actual with faster solution: {}\n", util::run(run_part3_faster, 3));

}
