use std::collections::{HashMap, VecDeque};
use itertools::Itertools;
use everybody_codes_util as util;
use util::grid::{Grid, Point};



type Pt = Point<isize, 2>;

fn run_part1(input_str: Vec<String>, _example: bool) -> String {
    let grid = Grid::from_string(input_str);
    let start: Pt = grid.filter_first(b'S');
    let altitude = 1000;
    let seconds_left = 100;
    let dirs = [Pt::new([1, 0]), Pt::new([-1, 0]), Pt::new([0, 1]), Pt::new([0, -1])];

    let mut hist = HashMap::new();
    // let mut queue = VecDeque::from_iter(vec!((curr, altitude, seconds_left, dir)).into_iter());
    let mut queue = VecDeque::new();
    for dir in dirs.into_iter() {
        queue.push_back((start, altitude, seconds_left, dir));
    }
    let mut highest = 0;

    while queue.len() > 0 {
        let (loc, alt, sec, dir) = queue.pop_front().unwrap();
        // println!("{:?}", (loc, alt, sec, dir) );
        if sec == 0 {
            if alt > highest {
                // println!("{:?}", (loc, alt, sec, dir));
                highest = alt;
            }
            continue
        }

        let mut neighbors = grid.get_neighbors_filter(loc, vec!(b'#'));
        neighbors = neighbors.into_iter().filter(|x| x.0 != loc - dir).collect_vec();

        for neighbor in neighbors {
            let new_alt = alt + match neighbor.1 {
                b'+' => 1,
                b'-' => -2,
                _ => -1
            };
            // if !hist.contains_key(&(neighbor, sec - 1, neighbor.0-loc)) {
            //     hist.insert((neighbor, sec - 1, neighbor.0-loc), new_alt);
            // } else {
            //     if hist[&(neighbor, sec - 1, neighbor.0 - loc)] >= new_alt {
            //         continue
            //     }
            //     hist.insert((neighbor, sec - 1, neighbor.0 - loc), new_alt);
            // }
            if !hist.contains_key(&(neighbor, neighbor.0-loc)) {
                hist.insert((neighbor, neighbor.0-loc), new_alt);
            } else {
                if hist[&(neighbor, neighbor.0 - loc)] >= new_alt {
                    continue
                }
                hist.insert((neighbor, neighbor.0 - loc), new_alt);
            }
            queue.push_back((neighbor.0, new_alt, sec - 1, neighbor.0 - loc))
        }
    }

    highest.to_string()
}

fn run_part2(input_str: Vec<String>, _example: bool) -> String {
    let grid = Grid::from_string(input_str);
    let start: Pt = grid.filter_first(b'S');
    let checks = [grid.filter_first(b'A'), grid.filter_first(b'B'), grid.filter_first(b'C')];
    let altitude = 10000;
    let dirs = [Pt::new([1, 0]), Pt::new([-1, 0]), Pt::new([0, 1]), Pt::new([0, -1])];

    let mut hist = HashMap::new();
    // let mut queue = VecDeque::from_iter(vec!((curr, altitude, seconds_left, dir)).into_iter());
    let mut queue = VecDeque::new();
    for dir in dirs.into_iter() {
        queue.push_back((start, altitude, 0, dir, [false, false, false]));
    }
    let mut fastest = 0;

    while queue.len() > 0 {
        let (loc, alt, sec, dir, vis) = queue.pop_front().unwrap();
        // println!("{:?}", (loc, alt, sec, dir, vis) );
        if vis.iter().all(|x| *x) && loc == start && alt >= 10000 {
            fastest = sec;
            break
        }

        let mut neighbors = grid.get_neighbors_filter(loc, vec!(b'#'));
        neighbors = neighbors.into_iter().filter(|x| x.0 != loc - dir).collect_vec();

        for neighbor in neighbors {
            let new_alt = alt + match neighbor.1 {
                b'+' => 1,
                b'-' => -2,
                _ => -1
            };

            let new_vis = if !checks.contains(&neighbor.0) { vis } else {
                if neighbor.0 == checks[0] {
                    [true, vis[1], vis[2]]
                } else if neighbor.0 == checks[1] {
                    if vis[0] {
                        [true, true, vis[2]]
                    } else { vis }
                } else {
                    if vis[0] && vis[1] {
                        [true, true, true]
                    } else { vis }
                }
            };

            // if !hist.contains_key(&(neighbor, sec - 1, neighbor.0-loc)) {
            //     hist.insert((neighbor, sec - 1, neighbor.0-loc), new_alt);
            // } else {
            //     if hist[&(neighbor, sec - 1, neighbor.0 - loc)] >= new_alt {
            //         continue
            //     }
            //     hist.insert((neighbor, sec - 1, neighbor.0 - loc), new_alt);
            // }
            if !hist.contains_key(&(neighbor, neighbor.0-loc, new_vis)) {
                hist.insert((neighbor, neighbor.0-loc, new_vis), new_alt);
            } else {
                if hist[&(neighbor, neighbor.0 - loc, new_vis)] >= new_alt {
                    continue
                }
                hist.insert((neighbor, neighbor.0 - loc, new_vis), new_alt);
            }
            queue.push_back((neighbor.0, new_alt, sec + 1, neighbor.0 - loc, new_vis))
        }
    }

    fastest.to_string()
}

fn run_algo_for(altitude: isize, grid: &Grid<u8>) -> isize {
    let start: Pt = grid.filter_first(b'S');
    let dirs = [Pt::new([1, 0]), Pt::new([-1, 0]), Pt::new([0, 1]), Pt::new([0, -1])];

    let mut hist = HashMap::new();
    let mut alt_hist = HashMap::new();  // double name :)
    let mut queue = VecDeque::new();
    for dir in dirs.into_iter() {
        queue.push_back((start, altitude, dir, 0));
    }
    let mut most_south = 0;
    let mult = grid.get_dims()[0] as isize;
    // let mut hist_wrap = HashMap::new();


    while queue.len() > 0 {
        let (loc, alt, dir, loops) = queue.pop_front().unwrap();

        if alt == 0 {
            if loc.0[0] + loops * mult > most_south {
                most_south = loc.0[0] + loops * mult;
            }
            continue
        }

        let mut neighbors: Vec<(Pt, u8, isize)> = grid.get_neighbors_ok(loc).into_iter().map(|x| (x.0, x.1, 0)).collect_vec();
        if loc.0[0] == mult - 1 {
            let wrap_pt = loc - Pt::new([loc.0[0], 0]);
            neighbors.push((wrap_pt, grid.get_pt(wrap_pt), 1));
        }
        neighbors = neighbors.into_iter().filter(|x| x.0 != loc - dir)
            .filter(|x| x.1 != b'#').collect_vec();


        for neighbor in neighbors.into_iter().rev() {
            let new_alt = alt + match neighbor.1 {
                b'+' => 1,
                b'-' => -2,
                _ => -1
            };
            if new_alt < 0 {
                continue
            }
            let actual_pt = neighbor.0 + Pt::new([(loops + neighbor.2) * mult, 0]);

            if !hist.contains_key(&(actual_pt, neighbor.0-loc)) {
                hist.insert((actual_pt, neighbor.0-loc), new_alt);
            } else {
                if hist[&(actual_pt, neighbor.0 - loc)] >= new_alt {
                    continue
                }
                hist.insert((actual_pt, neighbor.0 - loc), new_alt);
            }

            if !alt_hist.contains_key(&new_alt) {
                alt_hist.insert(new_alt, actual_pt.0[0]);
            } else {
                if alt_hist[&new_alt] > (actual_pt.0[0] + 3) {
                    continue
                }
                if alt_hist[&new_alt] < (actual_pt.0[0]) {
                    alt_hist.insert(new_alt, actual_pt.0[0]);
                }
            }
            queue.push_back((neighbor.0, new_alt, neighbor.0 - loc, loops + neighbor.2))
        }
    }
    most_south
}


fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    let grid = Grid::from_string(input_str);

    let nums = [100, 1000];
    let ans = nums.map(|x| run_algo_for(x, &grid));

    let slope = (ans[1] - ans[0]) / (nums[1] - nums[0]);
    let intercept = ans[1] - 1000*slope;

    (intercept + 384400 * slope).to_string()

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
    println!("Example 1: {}", util::run(run_part2, -2));
    println!("Example 2: {}", util::run(run_part2, -3));
    println!("Example 3: {}", util::run(run_part2, -4));
    println!("Actual: {}\n", util::run(run_part2, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example: {}", util::run(run_part3, -5));
    println!("Actual: {}\n", util::run(run_part3, 3));

}
