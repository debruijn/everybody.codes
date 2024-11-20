use std::cmp::min;
use itertools::Itertools;
use everybody_codes_util as util;

fn run_part1(input_str: Vec<String>) -> String {  // A: mod 3 = 0, B: mod 3 = 1
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
        // println!("{}, {}, {}", score_incr, height, power);
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
        let height = if example { 3- t.0} else {19 - t.0};
        let shooter = (dist + height) % 3;
        let power = (dist + height) / 3;
        let score_incr = (shooter + 1) * power;
        // println!("{}, {}, {}", score_incr, height, power);
        score += score_incr
    }

    score.to_string()
}

fn can_hit(mut meteor: (isize, isize), height: isize, power: isize, delay: isize) -> (bool, isize, isize, isize) {
    let mut t = 0;
    let mut proj = (height, 0);
    meteor = (meteor.1, meteor.0);
    loop {
        t += 1;
        // println!("{:?}, {:?}, {}", meteor, proj, t);
        if t > delay {
            if t - delay <= power {
                proj = (proj.0 + 1, proj.1 + 1);
            } else if t - delay <= power * 2 {
                proj = (proj.0, proj.1 + 1);
            } else {
                return if proj.0 == meteor.0 {
                    if meteor.1 > proj.1 && (meteor.1 - proj.1) % 2 == 0 && (meteor.1 - proj.1) <= proj.0 * 2 {
                        (true, proj.0 - (meteor.1 - proj.1) / 2, 0, t)
                    } else {
                        (false, 0, meteor.1 - proj.1, t)
                    }
                } else {
                    (false, meteor.0 - proj.0, meteor.1 - proj.1, t)
                }
                // proj = (proj.0 - 1, proj.1 + 1)
            }
        }
        meteor = (meteor.0-1, meteor.1-1);
        if proj == meteor {
            return (true, meteor.0, 0, t) // power * (height + 1)
        }
        // if proj.1 > meteor.1 {
        //     return (false, meteor.1 - proj.1)
        // }
    }
}

// 1 -> after 2, 1 higher
// 2 -> after 4, 2 higher
// etc
// at that point: should have same height and even diff
// for 1: -> if meteor after 2 is at (x2, y2), and we at (xm, ym), then ym==y2, and x2-xm is even and smaller than 2*xm
// if not ym==y2, but, say, 5 higher, then delay by 5 and redo test for this power
// continue increasing power until match, or all over
//  over:

fn solve_meteor(meteor: &(isize, isize), iter: usize) -> isize {let mut this_best_height = 0;
    let mut this_worst_val = 100000000;
    let mut cand_res = (0, 0, 0);
    for cat in 0..3 {
        // let mut try_pow = 1;
        for try_pow in 1..3000{
            // loop {
            let this_res = can_hit(*meteor, cat, try_pow, 0);
            // println!("{:?}, {}, {}, {:?}, {}, {}, {}", meteor, cat, try_pow, this_res, 0, this_worst_val, this_best_height);
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
                // break
            }
            if this_res.1 > 0 && this_res.2 > 0{
                let t = this_res.1;
                let this_res = can_hit(*meteor, cat, try_pow, t);
                // println!("{:?}, {}, {}, {:?}, {}, {}, {}", meteor, cat, try_pow, this_res, t, this_worst_val, this_best_height);
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
                    // break
                }
                // try_pow += 1;
                continue
            }
            if this_res.1 >=0 && this_res.2 <= 0 {
                for t in 1..this_res.3 {
                    // let t = this_res.1;
                    let this_res = can_hit(*meteor, cat, try_pow, t);
                    // println!("{:?}, {}, {}, {:?}, {}, {}, {}", meteor, cat, try_pow, this_res, t, this_worst_val, this_best_height);
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
                        // break
                    }
                }
                // try_pow += 1;
                continue
            }
            break
        }
    }
    println!("{iter}: {:?}, {}, {:?}", meteor, this_worst_val, cand_res);
    this_worst_val
}


fn run_part3(input_str: Vec<String>) -> String {
    let meteors: Vec<(isize, isize)> = input_str.iter().map(|x| {let temp = x.split_once(' ').unwrap();
        (temp.0.parse().unwrap(), temp.1.parse().unwrap())}).collect_vec();

    let mut value = 0;
    for (iter, meteor) in meteors.iter().enumerate() {
        let this_worst_val = solve_meteor(meteor, iter);
        value += this_worst_val;
    }

    // for each meteor:
    // - where is meteor at time t (t = ...)?
    // - where is projectile if launched at t (t=0, 1, 2, 3) from A, B, or C?
    // -
    value.to_string()
}


fn run_part3_alt(input_str: Vec<String>) -> String {
    let meteors: Vec<(isize, isize)> = input_str.iter().map(|x| {let temp = x.split_once(' ').unwrap();
        (temp.0.parse().unwrap(), temp.1.parse().unwrap())}).collect_vec();

    let mut value = 0;
    for (iter, meteor) in meteors.iter().enumerate() {
        let mut this_best_height = 0;
        let mut this_worst_val = 100000000;
        let mut cand_res = (0, 0, 0);
        for cat in 0..3 {
            // let mut try_pow = 1;
            for try_pow in 1..2000 {
                // loop {
                for t in 0..2 {
                    // let t = 0;
                    let this_res = can_hit(*meteor, cat, try_pow, t);
                    // println!("{:?}, {}, {}, {:?}, {}, {}, {}", meteor, cat, try_pow, this_res, 0, this_worst_val, this_best_height);
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
                        // break
                    }
                }
                let t = try_pow - (meteor.0 - meteor.1 + cat);
                if t >= 0 {
                    let this_res = can_hit(*meteor, cat, try_pow, t);
                    // println!("{:?}, {}, {}, {:?}, {}, {}, {}", meteor, cat, try_pow, this_res, 0, this_worst_val, this_best_height);
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
                        // break
                    }
                }
            }
        }
        println!("{iter}: {:?}, {}, {:?}", meteor, this_worst_val, cand_res);
        if this_worst_val == 100000000 {
            this_worst_val = solve_meteor(meteor, iter);
        }
        value += this_worst_val;
    }

    // for each meteor:
    // - where is meteor at time t (t = ...)?
    // - where is projectile if launched at t (t=0, 1, 2, 3) from A, B, or C?
    // -
    value.to_string()
}

fn main() {
    // Plan:
    // - Automatically iterate over all three parts
    // - Adjust to make it clearer that negative int is example
    // - Create nice output print function, printing part and whether it is example or actual
    // - Create template for a joint function that solves all 3

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
    println!("Example: {}", run_part3(input_str.clone()));
    println!("Example: {}", run_part3_alt(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3_alt(input_str));  //1256270  // 1022979  // 752379  // 708877

}
