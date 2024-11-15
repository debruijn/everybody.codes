use everybody_codes_util as util;
use itertools::Itertools;

fn process_input(input_str: Vec<String>) -> Vec<isize> {
    input_str.iter().map(|x| x.parse().unwrap()).collect_vec()
}

fn dp(up_to: usize, beetles: Vec<usize>) -> Vec<usize> {
    let mut mapping = vec![up_to; up_to]; // max value for max elements
    mapping[0] = 0;

    for &beetle in beetles.iter() {
        for i in beetle..up_to {
            mapping[i] = mapping[i].min(mapping[i - beetle] + 1);
        }
    }
    mapping
}

fn run_part1(input_str: Vec<String>) -> String {
    let beetles = vec![1, 3, 5, 10];
    let res: Vec<usize> = process_input(input_str)
        .iter()
        .map(|x| *x as usize)
        .collect_vec();
    let up_to = 1 + (&res).iter().max().unwrap();
    let mapping = dp(up_to, beetles);
    res.iter().map(|x| mapping[*x]).sum::<usize>().to_string()
}

fn run_part2(input_str: Vec<String>) -> String {
    let beetles = vec![1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    let res: Vec<usize> = process_input(input_str)
        .iter()
        .map(|x| *x as usize)
        .collect_vec();
    let up_to = 1 + (&res).iter().max().unwrap();
    let mapping = dp(up_to, beetles);
    res.iter().map(|x| mapping[*x]).sum::<usize>().to_string()
}

fn try_split(num: usize, mapping: Vec<usize>) -> usize {
    (num / 2 - 51..num / 2 + 1)
        .filter(|x| x.abs_diff(num - x) <= 100)
        .map(|x| mapping[x] + mapping[num - x])
        .min()
        .unwrap()
}

fn run_part3(input_str: Vec<String>) -> String {
    let beetles = vec![
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    let res: Vec<usize> = process_input(input_str)
        .iter()
        .map(|x| *x as usize)
        .collect_vec();
    let up_to = 1 + (&res).iter().max().unwrap();
    let mapping = dp(up_to, beetles);
    res.iter()
        .map(|x| try_split(*x, mapping.clone()))
        .sum::<usize>()
        .to_string()
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
