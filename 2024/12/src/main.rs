use everybody_codes_util as util;
use itertools::Itertools;
use std::cmp::min;

fn run_part1(input_str: Vec<String>) -> String {
    // A: mod 3 = 0, B: mod 3 = 1
    let mut all_ts = Vec::new();
    for row in input_str.iter().enumerate() {
        for el in row.1.chars().enumerate() {
            if el.1 == 'T' {
                all_ts.push((row.0, el.0))
            }
        }
    }

    let mut score = 0;
    for t in all_ts.iter() {
        let dist = t.1 - 1;
        let height = 3 - t.0;
        let shooter = (dist + height) % 3;
        let power = (dist + height) / 3;
        let score_incr = (shooter + 1) * power;
        score += score_incr
    }

    score.to_string()
}

fn run_part2(input_str: Vec<String>, example: bool) -> String {
    let mut all_ts = Vec::new();
    for row in input_str.iter().enumerate() {
        for el in row.1.chars().enumerate() {
            if el.1 == 'T' {
                all_ts.push((row.0, el.0))
            }
            if el.1 == 'H' {
                all_ts.push((row.0, el.0));
                all_ts.push((row.0, el.0));
            }
        }
    }

    let mut score = 0;
    for t in all_ts.iter() {
        let dist = t.1 - 1;
        let height = if example { 3 - t.0 } else { 19 - t.0 };
        let shooter = (dist + height) % 3;
        let power = (dist + height) / 3;
        let score_incr = (shooter + 1) * power;
        score += score_incr
    }

    score.to_string()
}

fn can_hit(
    mut meteor: (isize, isize),
    height: isize,
    power: isize,
    delay: isize,
) -> (bool, isize, isize, isize) {
    let mut t = 0;
    let mut proj = (height, 0);
    meteor = (meteor.1, meteor.0);
    loop {
        let diff = 1;
        t += diff;
        // println!("{:?}, {:?}, {}", meteor, proj, t);
        if t > delay {
            if t - delay <= power {
                proj = (proj.0 + diff, proj.1 + diff);
            } else if t - delay <= power * 2 {
                proj = (proj.0, proj.1 + diff);
            } else {
                return if proj.0 == meteor.0 {
                    if meteor.1 > proj.1
                        && (meteor.1 - proj.1) % 2 == 0
                        && (meteor.1 - proj.1) <= proj.0 * 2
                    {
                        (true, proj.0 - (meteor.1 - proj.1) / 2, 0, t)
                    } else {
                        (false, 0, meteor.1 - proj.1, t)
                    }
                } else {
                    (false, meteor.0 - proj.0, meteor.1 - proj.1, t)
                };
            }
        }
        meteor = (meteor.0 - 1, meteor.1 - 1);
        if proj == meteor {
            return (true, meteor.0, 0, t); // power * (height + 1)
        }
    }
}

fn solve_meteor(meteor: &(isize, isize), iter: usize) -> isize {
    let mut this_best_height = 0;
    let mut this_worst_val = 100000000;
    let mut cand_res = (0, 0, 0);
    for cat in 0..3 {
        for try_pow in 1..3000 {
            let this_res = can_hit(*meteor, cat, try_pow, 0);
            if this_res.0 {
                if this_res.1 > this_best_height {
                    this_worst_val = try_pow * (cat + 1);
                    this_best_height = this_res.1;
                    cand_res = (cat, try_pow, 0);
                } else if this_res.1 == this_best_height {
                    if this_worst_val > try_pow * (cat + 1) {
                        cand_res = (cat, try_pow, 0);
                    }
                    this_worst_val = min(try_pow * (cat + 1), this_worst_val);
                }
            }
            if this_res.1 > 0 && this_res.2 > 0 {
                let t = this_res.1;
                let this_res = can_hit(*meteor, cat, try_pow, t);
                if this_res.0 {
                    if this_res.1 > this_best_height {
                        this_worst_val = try_pow * (cat + 1);
                        this_best_height = this_res.1;
                        cand_res = (cat, try_pow, t);
                    } else if this_res.1 == this_best_height {
                        if this_worst_val > try_pow * (cat + 1) {
                            cand_res = (cat, try_pow, t);
                        }
                        this_worst_val = min(try_pow * (cat + 1), this_worst_val);
                    }
                }
                continue;
            }
            if this_res.1 >= 0 && this_res.2 <= 0 {
                for t in 1..this_res.3 {
                    let this_res = can_hit(*meteor, cat, try_pow, t);
                    if this_res.0 {
                        if this_res.1 > this_best_height {
                            this_worst_val = try_pow * (cat + 1);
                            this_best_height = this_res.1;
                            cand_res = (cat, try_pow, t);
                        } else if this_res.1 == this_best_height {
                            if this_worst_val > try_pow * (cat + 1) {
                                cand_res = (cat, try_pow, t);
                            }
                            this_worst_val = min(try_pow * (cat + 1), this_worst_val);
                        }
                    }
                }
                continue;
            }
            break;
        }
    }
    println!("{iter}: {:?}, {}, {:?}", meteor, this_worst_val, cand_res);
    this_worst_val
}

fn run_part3(input_str: Vec<String>) -> String {
    let meteors: Vec<(isize, isize)> = input_str
        .iter()
        .map(|x| {
            let temp = x.split_once(' ').unwrap();
            (temp.0.parse().unwrap(), temp.1.parse().unwrap())
        })
        .collect_vec();

    let mut value = 0;
    for (iter, meteor) in meteors.iter().enumerate() {
        let this_worst_val = solve_meteor(meteor, iter);
        value += this_worst_val;
    }

    value.to_string()
}

fn run_part3_alt(input_str: Vec<String>) -> String {
    let meteors: Vec<(isize, isize)> = input_str
        .iter()
        .map(|x| {
            let temp = x.split_once(' ').unwrap();
            (temp.0.parse().unwrap(), temp.1.parse().unwrap())
        })
        .collect_vec();

    let mut value = 0;
    for (iter, meteor) in meteors.iter().enumerate() {
        let mut this_best_height = 0;
        let mut this_worst_val = 100000000;
        for cat in 0..3 {
            for try_pow in meteor.0 / 6 - 1..meteor.1 / 2 + 2 {
                for t in 0..2 {
                    let this_res = can_hit(*meteor, cat, try_pow, t);
                    if this_res.0 {
                        if this_res.1 > this_best_height {
                            this_worst_val = try_pow * (cat + 1);
                            this_best_height = this_res.1;
                        } else if this_res.1 == this_best_height {
                            this_worst_val = min(try_pow * (cat + 1), this_worst_val);
                        }
                    }
                }
                let t = try_pow - (meteor.0 - meteor.1 + cat);
                if t >= 0 {
                    let this_res = can_hit(*meteor, cat, try_pow, t);
                    if this_res.0 {
                        if this_res.1 > this_best_height {
                            this_worst_val = try_pow * (cat + 1);
                            this_best_height = this_res.1;
                        } else if this_res.1 == this_best_height {
                            this_worst_val = min(try_pow * (cat + 1), this_worst_val);
                        }
                    }
                }
            }
        }
        if this_worst_val == 100000000 {
            this_worst_val = solve_meteor(meteor, iter);
        }
        value += this_worst_val;
    }

    value.to_string()
}

fn update_hi_score(highest: isize, score: isize, y: isize, this_score: isize) -> (isize, isize) {
    if y > highest {
        (y, this_score)
    } else if y == highest {
        (highest, min(score, this_score))
    } else {
        (highest, score)
    }
}

fn check_start_meteor(start: isize, meteor: &(isize, isize)) -> (isize, isize) {
    let mut highest = -1;
    let mut score = -1;

    let (x, y) = (meteor.0 / 2, meteor.1 - (meteor.0 - meteor.0 / 2));

    if y == x + start {
        // Hits on incline
        let this_score = x * (start + 1);
        (highest, score) = update_hi_score(highest, score, y, this_score);
    }
    if x >= y - start && x <= 2 * (y - start) {
        // Hits on flat part
        let this_score = (y - start) * (start + 1);
        (highest, score) = update_hi_score(highest, score, y, this_score);
    }
    if x >= 2 * (y - start + x) / 3 && (y - start + x) % 3 == 0 {
        // Hits on decline
        let this_score = (y - start + x) / 3 * (start + 1);
        (highest, score) = update_hi_score(highest, score, y, this_score);
    }
    (highest, score)
}

fn run_part3_calc(input_str: Vec<String>) -> String {
    let meteors: Vec<(isize, isize)> = input_str
        .iter()
        .map(|x| {
            let temp = x.split_once(' ').unwrap();
            (temp.0.parse().unwrap(), temp.1.parse().unwrap())
        })
        .collect_vec();
    let mut value = 0;
    for meteor in meteors.iter() {
        let mut highest = -1;
        let mut score = -1;
        for start in 0..3 {
            // try all three start points
            let (this_highest, this_score) = check_start_meteor(start, meteor);
            (highest, score) = update_hi_score(highest, score, this_highest, this_score);
        }
        value += score;
    }
    value.to_string()
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
    println!("Example: {}", run_part2(input_str, true));
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str, false));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3_calc(input_str.clone()));
    println!("Example: {}", run_part3(input_str.clone()));
    println!("Example: {}", run_part3_alt(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3_calc(input_str.clone()));
}
