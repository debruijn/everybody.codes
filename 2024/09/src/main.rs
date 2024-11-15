use std::cmp::min;
use itertools::Itertools;
use everybody_codes_util as util;

fn process_input(input_str: Vec<String>) -> Vec<isize> {
    input_str.iter().map(|x| x.parse().unwrap()).collect_vec()
}

fn get_lowest_num_beetles(num: isize) -> isize {
    if num > 30 {
        return get_lowest_num_beetles(num % 30) + (num / 30) * 3
    }

    let beetles = vec!(1, 3, 5, 10);
    let curr_min = num / beetles.iter().max().unwrap();
    for i in curr_min..(num+1) {
        for combo in beetles.iter().combinations_with_replacement(i as usize) {
            if combo.clone().into_iter().sum::<isize>() == num {
                // println!("{}: {:?}, {}", num, combo, i);
                return i
            }
        }
    }
    curr_min
}

fn run_part1(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    res.iter().map(|x| get_lowest_num_beetles(*x)).sum::<isize>().to_string()
}

fn get_lowest_num_beetles_p2(num: isize) -> isize {
    if num > 900 {
        return get_lowest_num_beetles_p2(num % 900) + (num / 900) * 30
    }

    let beetles = vec!(1, 3, 5, 10, 15, 16, 20, 24, 25, 30);
    let curr_min = num / beetles.iter().max().unwrap();
    for i in curr_min..(num+1) {
        for combo in beetles.iter().combinations_with_replacement(i as usize) {
            if combo.clone().into_iter().sum::<isize>() == num {
                // println!("{}: {:?}, {}", num, combo, i);
                return i
            }
        }
    }
    curr_min
}

fn algoalgoalgo(num: isize) -> isize {
    if num <= 30 {
        return get_lowest_num_beetles_p2(num)
    }
    let rem = get_lowest_num_beetles_p2(num % 30);
    let mut ub = rem + (num / 30);

    for i in 1..5 {
        let this = get_lowest_num_beetles_p2(num % 30 + 30*i) + (num/30)-i;
        ub = min(ub, this)
    }
    // println!("{}: {}", num, ub);
    ub
    }


fn run_part2(input_str: Vec<String>) -> String {
    let res = process_input(input_str);
    res.iter().map(|x| algoalgoalgo(*x)).sum::<isize>().to_string()

}


fn try_split(num: usize, mapping: Vec<usize>) -> usize {
    (num/2-51..num/2+1).filter(|x| x.abs_diff(num-x)<=100).map(|x| mapping[x] + mapping[num - x]).min().unwrap()
}

fn dp_new(notes: Vec<usize>) -> Vec<usize> {
    let max = 1 + notes.iter().max().unwrap();
    let beetles = vec!(1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101);

    let mut mapping = vec![max; max];  // max value for max elements
    mapping[0] = 0;

    for &beetle in beetles.iter().skip(1) {
        for i in beetle..max {
            mapping[i] = mapping[i].min(mapping[i - beetle] + 1);
        }
    }

    mapping
}

fn run_part3(input_str: Vec<String>) -> String {
    let res: Vec<usize> = process_input(input_str).iter().map(|x| *x as usize).collect_vec();
    let mapping = dp_new(res.clone());
    res.iter().map(|x| try_split(*x, mapping.clone())).sum::<usize>().to_string()
}

fn main() {
    // Part 1: example and actual 8722 8713 5585 5168 5165 5160 150495 150498 150496 150494 150493
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
    println!("Example: {}", run_part3(input_str));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str));

}
