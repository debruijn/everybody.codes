use everybody_codes_util as util;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::Sub;

fn transpose(original: &Vec<String>) -> Vec<String> {
    assert!(!original.is_empty());
    let mut transposed = (0..original[0].len()).map(|_| vec![]).collect::<Vec<_>>();

    for original_row in original {
        for (item, transposed_row) in original_row.chars().into_iter().zip(&mut transposed) {
            transposed_row.push(item);
        }
    }
    transposed
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect_vec()
}

fn chars_to_set(chars: Vec<char>) -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    for k in chars[0..2].iter() {
        set.insert(*k);
    }
    for k in chars[6..8].iter() {
        set.insert(*k);
    }
    set
}

fn intersect_row_col(row: &HashSet<char>, col: HashSet<char>) -> char {
    let intersect = row.intersection(&col);
    let intersect_val = intersect.filter(|x| **x != '?').collect_vec();
    let this_char = if intersect_val.len() == 1 {
        **intersect_val.iter().next().unwrap()
    } else {
        '?'
    };
    this_char
}

fn get_runic_word(input_str: Vec<String>) -> String {
    let transposed_str = transpose(&input_str);
    let mut res = String::new();
    for i in 2..6 {
        let chars = input_str[i].chars().collect_vec();
        let row = chars_to_set(chars);
        for j in 2..6 {
            let chars = transposed_str[j].chars().collect_vec();
            let col = chars_to_set(chars);
            let this_char = intersect_row_col(&row, col);
            res += &*this_char.to_string();
        }
    }
    res
}

fn runic_power(runic_word: String) -> usize {
    if runic_word.bytes().contains(&b'?') {
        return 0;
    }
    runic_word
        .bytes()
        .enumerate()
        .map(|x| (x.0 + 1) * (x.1 - b'A' + 1) as usize)
        .sum()
}

fn grid_to_puzzle(input_str: &Vec<String>, i: usize, j: usize, step: usize) -> Vec<String> {
    let this_string = input_str[i * step..(i * step) + 8]
        .iter()
        .map(|x| {
            let chars = x.chars().collect_vec();
            chars[j * step..(j * step) + 8].iter().join("")
        })
        .collect_vec();
    this_string
}

fn run_part1(input_str: Vec<String>) -> String {
    get_runic_word(input_str)
}

fn run_part2(input_str: Vec<String>, example: bool) -> String {
    let dims = if example { (1, 1) } else { (7, 15) };

    let mut sum_power = 0;
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            let this_string = grid_to_puzzle(&input_str, i, j, 9);
            sum_power += runic_power(get_runic_word(this_string));
        }
    }
    sum_power.to_string()
}

fn get_runic_word_with_marks(mut input_str: Vec<String>) -> Vec<String> {
    let func_input = input_str.clone();
    let transposed_str = transpose(&input_str);
    let mut res = [['?'; 4]; 4];
    let mut mark_locs = Vec::new(); // mark the locations with a ?

    // First: normal solve as much as possible but mark question marks
    for i in 2..6 {
        let chars = input_str[i].chars().collect_vec();
        let row = chars_to_set(chars);
        for j in 2..6 {
            let chars = transposed_str[j].chars().collect_vec();
            let col = chars_to_set(chars);
            let this_char = intersect_row_col(&row, col);
            if this_char != '?' {
                res[i - 2][j - 2] = this_char;
            } else {
                mark_locs.push((i, j))
            }
        }
    }

    // Then: go over all ?s and see if they can be filled in (by row or by col)
    for loc in mark_locs.iter() {
        let outer_row = HashSet::from_iter(input_str[loc.0].chars().filter(|x| *x != '.'));
        let outer_col: HashSet<char> =
            HashSet::from_iter(transposed_str[loc.1].chars().filter(|x| *x != '.'));

        if outer_row.contains(&'?') && outer_row.len() >= 4 {
            let inside = HashSet::from_iter(res.iter().map(|x| x[loc.1 - 2]));
            let overlap = outer_col.sub(&inside.to_owned());
            let new = if overlap.len() == 1 {
                overlap.iter().next().unwrap()
            } else {
                &'?'
            };
            if !res.iter().any(|x| x.contains(new)) {
                input_str[loc.0] = input_str[loc.0].replace('?', &new.to_string());
            }
        } else if outer_col.contains(&'?') && outer_col.len() >= 4 {
            let inside = HashSet::from(res[loc.0 - 2]);
            let overlap = outer_row.sub(&inside.to_owned());
            let new = if overlap.len() == 1 {
                overlap.iter().next().unwrap()
            } else {
                &'?'
            };
            for j in 0..8 {
                if input_str[j].chars().nth(loc.1).unwrap() == '?' {
                    let mut temp = input_str[j].chars().collect_vec();
                    temp[loc.1] = *new;
                    input_str[j] = temp.iter().join("");
                }
            }
        }
    }
    // ONLY return updated input_str if fully solved (this assumption I missed for a long time)
    if !input_str.iter().any(|x| x.contains('?')) {
        input_str
    } else {
        func_input
    }
}

fn run_part3(mut input_str: Vec<String>, example: bool) -> String {
    let dims = if example { (2, 2) } else { (10, 20) };
    let mut total_marks: usize = input_str
        .iter()
        .map(|x| x.chars().filter(|y| y == &'?').count())
        .sum();

    loop {
        for i in 0..dims.0 {
            for j in 0..dims.1 {
                let mut this_string = grid_to_puzzle(&input_str, i, j, 6);
                this_string = get_runic_word_with_marks(this_string);
                for (k, ind) in (i * 6..(i + 1) * 6 + 2).enumerate() {
                    let mut chars = input_str[ind].chars().collect_vec();
                    let mut new_chars = this_string[k].chars();
                    for n in j * 6..(j + 1) * 6 + 2 {
                        chars[n] = new_chars.next().unwrap();
                    }
                    input_str[ind] = chars.iter().join("");
                }
            }
        }

        let new_marks = input_str
            .iter()
            .map(|x| x.chars().filter(|y| y == &'?').count())
            .sum();
        if new_marks == total_marks {
            break;
        } else {
            total_marks = new_marks
        }
    }

    // After no more changes
    let mut sum_power: usize = 0;
    for i in 0..dims.0 {
        for j in 0..dims.1 {
            let this_string = grid_to_puzzle(&input_str, i, j, 6);
            sum_power += runic_power(get_runic_word(this_string));
        }
    }

    sum_power.to_string()
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    let input_str = util::read_input(-1);
    println!("Example: {}", run_part1(input_str));
    let input_str = util::read_input(1);
    println!("Actual: {}\n", run_part1(input_str));

    // Part 2: example and actual
    println!("Part 2");
    let input_str = util::read_input(-2);
    println!("Example: {}", run_part2(input_str, true));
    let input_str = util::read_input(2);
    println!("Actual: {}\n", run_part2(input_str, false));

    // Part 3: example and actual
    println!("Part 3");
    let input_str = util::read_input(-3);
    println!("Example: {}", run_part3(input_str, true));
    let input_str = util::read_input(3);
    println!("Actual: {}\n", run_part3(input_str, false));
}
