use everybody_codes_util as util;
use everybody_codes_util::grid::{Grid, Point};
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashMap;
use std::isize::MAX;

fn get_val(el: char) -> isize {
    if el == 'E' || el == 'S' || el == 'T' {
        0
    } else {
        el.to_digit(10).unwrap() as isize
    }
}

fn get_dist(x1: isize, x2: isize) -> isize {
    if x2 == x1 {
        0
    } else if x2 > x1 {
        get_dist(x2, x1)
    } else {
        min(x1 - x2, x2 + 10 - x1)
    }
}

fn find_solution(input_str: Vec<String>, stop_at: isize, mut best_at_point: HashMap<Point<isize,2>, isize>) -> (isize, HashMap<Point<isize, 2>, isize>) {
    let grid: Grid<char> = Grid::from_string(input_str);
    let loc = grid.filter_key('S').into_iter().next().unwrap();
    let target = grid.filter_key('E').into_iter().next().unwrap();

    let mut curr_t = 0;
    let mut queue: HashMap<isize, Vec<Point<isize, 2>>> = HashMap::new();
    queue.insert(0, vec![loc]);
    // println!("{:?}", loc);
    // let mut best_at_point: HashMap<Point<isize, 2>, isize> = HashMap::new();
    while curr_t < stop_at {
        if !queue.contains_key(&curr_t) || queue[&curr_t].len() == 0 {
            curr_t += 1;
            // println!("T to {curr_t}, queue keys: {:?}", queue.keys());
            continue;
        }
        // println!("T stays at {curr_t}, queue count: {:?}", queue.iter().map(|x| x.1.len()).collect_vec());
        // println!("{:?}", queue);
        let pt = queue.get_mut(&curr_t).unwrap().pop().unwrap();
        if pt == target {
            break;
        }

        let this_val = get_val(grid.get_pt(pt));
        let others = grid.get_neighbors_ok(pt);
        // println!("{:?}", others);
        // println!("{:?}", queue);
        for other in others.into_iter() {
            if other.1 == '#' {
                continue;
            }
            let o_val = get_val(other.1);
            // println!("{:?}, {:?}, {}, {}, {}", pt, other.0, curr_t, this_val, o_val);
            let o_t = curr_t + get_dist(this_val, o_val) + 1;
            if best_at_point.keys().contains(&other.0) {
                if best_at_point[&other.0] <= o_t {
                    continue;
                }
            }
            best_at_point.insert(other.0, o_t);
            queue.entry(o_t).or_default().push(other.0)
        }
    }
    (curr_t, best_at_point)
}

fn run_part1(input_str: Vec<String>) -> String {
    find_solution(input_str, isize::MAX, HashMap::new()).0.to_string()
}

fn run_part2(input_str: Vec<String>) -> String {
    find_solution(input_str, isize::MAX, HashMap::new()).0.to_string()
}

fn replace_t_to_s(input_str: &Vec<String>, i: usize, j: usize) -> Vec<String> {
    let mut temp_str = input_str.clone();
    let mut temp_row: Vec<char> = temp_str[i].chars().collect();
    temp_row[j] = 'S';
    temp_str[i] = temp_row.into_iter().join("");
    temp_str
}

fn run_part3(input_str: Vec<String>) -> String {
    // Replace all S with T, temporarily
    let adj_str = input_str.iter().map(|x| x.replace('S', "T")).collect_vec();

    // Loop over all T, and replace the chosen one with S, to get score
    let dims = (input_str.len(), input_str[0].chars().count());
    let (mut best_score, mut best_at_point) = find_solution(replace_t_to_s(&adj_str, dims.0/2, 0), isize::MAX, HashMap::new());
    println!("{}",best_score);
    (best_score, best_at_point) = find_solution(replace_t_to_s(&adj_str, dims.0/2, dims.1-1), best_score, best_at_point);
    println!("{}",best_score);
    (best_score, best_at_point)  = find_solution(replace_t_to_s(&adj_str, 0, dims.1/2), best_score, best_at_point);
    println!("{}",best_score);
    (best_score, best_at_point)  = find_solution(replace_t_to_s(&adj_str, dims.0-1, dims.1/2), best_score, best_at_point);
    println!("{}",best_score);


    for (i, row) in adj_str.iter().enumerate() {
        for (j, el) in row.chars().enumerate() {
            if el == 'T' {
                // println!{"{:?}", temp_str}
                let (this_score, _best_at_point) = find_solution(replace_t_to_s(&adj_str, i, j), best_score, best_at_point);
                best_at_point = _best_at_point;
                if this_score < best_score {
                    best_score = this_score;
                    println!("{}, {}, {}", i, j, best_score)
                }
            }
        }
    }
    best_score.to_string()
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
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));
}
