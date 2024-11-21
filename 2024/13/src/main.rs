use everybody_codes_util as util;
use everybody_codes_util::grid::{Grid, Point};
use itertools::Itertools;
use std::cmp::min;
use std::collections::HashMap;

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

fn find_solution(
    input_str: Vec<String>,
    stop_at: isize,
    mut best_at_point: HashMap<Point<isize, 2>, isize>,
) -> (isize, HashMap<Point<isize, 2>, isize>) {
    let grid: Grid<char> = Grid::from_string(input_str);
    let target = grid.filter_key('E');
    let loc = grid.filter_key('S').into_iter().next().unwrap();

    let mut curr_t = 0;
    let mut queue: HashMap<isize, Vec<Point<isize, 2>>> = HashMap::new();
    queue.insert(0, vec![loc]);
    while curr_t < stop_at {
        if !queue.contains_key(&curr_t) || queue[&curr_t].len() == 0 {
            curr_t += 1;
            continue;
        }
        let pt = queue.get_mut(&curr_t).unwrap().pop().unwrap();
        if target.contains(&pt) {
            break;
        }

        let this_val = get_val(grid.get_pt(pt));
        let others = grid.get_neighbors_ok(pt);

        for other in others.into_iter() {
            if other.1 == '#' {
                continue;
            }
            let o_val = get_val(other.1);
            let o_t = curr_t + get_dist(this_val, o_val) + 1;
            if o_t >= stop_at {
                continue;
            }
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
    find_solution(input_str, isize::MAX, HashMap::new())
        .0
        .to_string()
}

fn run_part2(input_str: Vec<String>) -> String {
    find_solution(input_str, isize::MAX, HashMap::new())
        .0
        .to_string()
}

fn replace_t_to_s(input_str: &Vec<String>, i: usize, j: usize) -> Vec<String> {
    let mut temp_str = input_str.clone();
    let mut temp_row: Vec<char> = temp_str[i].chars().collect();
    temp_row[j] = 'S';
    temp_str[i] = temp_row.into_iter().join("");
    temp_str
}

fn run_part3(input_str: Vec<String>) -> String {
    let dims = (input_str.len(), input_str[0].chars().count());
    let center = Point::new([dims.0 / 2, dims.1 / 2]);

    // Replace all S with T, temporarily, and sort by distance to center
    let adj_str = input_str.iter().map(|x| x.replace('S', "T")).collect_vec();
    let mut all_ts: Vec<Point<usize, 2>> = adj_str
        .iter()
        .enumerate()
        .map(|x| {
            x.1.chars()
                .enumerate()
                .filter(|y| y.1 == 'T')
                .map(move |y| Point::new([x.0, y.0]))
        })
        .flatten()
        .collect_vec();
    all_ts = all_ts
        .into_iter()
        .sorted_by_key(|x| x.manhattan_dist(center))
        .collect_vec();

    // Loop over all points and update best_score and best_at_point along the way
    let mut best_score = isize::MAX;
    let mut best_at_point = HashMap::new();
    for pt in all_ts.iter() {
        let this_str = replace_t_to_s(&adj_str, pt.get()[0], pt.get()[1]);
        let (this_score, _best_at_point) = find_solution(this_str, best_score, best_at_point);
        best_at_point = _best_at_point;
        best_score = min(best_score, this_score)
    }
    best_score.to_string()
}

fn run_part3_fast(input_str: Vec<String>) -> String {
    // Swap around S and E and keep going until you find an E (originally S) due to symmetry of move
    let input_str = input_str
        .iter()
        .map(|x| {
            x.chars()
                .map(|y| match y {
                    'S' => 'E',
                    'E' => 'S',
                    _ => y,
                })
                .join("")
        })
        .collect_vec();

    find_solution(input_str, isize::MAX, HashMap::new())
        .0
        .to_string()
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
    let input_str = util::read_input(-3);
    println!("Example (fast): {}", run_part3_fast(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3_fast(input_str));
}
