use everybody_codes_util as util;
use itertools::Itertools;
use std::collections::HashSet;

fn run_part1(input_str: Vec<String>) -> String {
    let words = input_str[0][6..].split(',');

    let mut res = Vec::new();
    for txt in input_str[2..].iter() {
        let this_res: usize = words
            .clone()
            .into_iter()
            .map(|x| txt.match_indices(x).collect_vec().len())
            .sum();
        res.push(this_res.to_string());
    }
    res.join(", ")
}

fn run_part2(input_str: Vec<String>) -> String {
    // Uncleaned version -- compare with run_all below for faster cleaner version

    let words = input_str[0][6..].split(',');
    let rev_words = words.clone().map(|x| x.chars().rev().collect::<String>());

    let mut res = Vec::new();
    for txt in input_str[2..].iter() {
        let mut inds = HashSet::new();

        for word in words.clone().into_iter() {
            let matches = txt.match_indices(word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert(i);
                }
            }
            let matches = txt.rmatch_indices(word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert(i);
                }
            }
        }

        for word in rev_words.clone().into_iter() {
            let matches = txt.match_indices(&word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert(i);
                }
            }
            let matches = txt.rmatch_indices(&word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert(i);
                }
            }
        }

        res.push(inds.len());
    }
    res.iter().sum::<usize>().to_string()
}

fn transpose(original: Vec<&String>) -> Vec<String> {
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

fn run_part3<'a>(input_str: Vec<String>) -> String {
    // Uncleaned version -- compare with run_all below for faster cleaner version

    let words = input_str[0][6..].split(',');
    let rev_words = words.clone().map(|x| x.chars().rev().collect::<String>());

    let mut inds = HashSet::new();

    for (h, txt) in input_str[2..].iter().enumerate() {
        for word in words.clone().into_iter() {
            let this_txt = String::new() + txt + &txt[..word.len() - 1];
            let matches = this_txt.match_indices(word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((h, i % txt.len()));
                }
            }
            let matches = this_txt.rmatch_indices(word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((h, i % txt.len()));
                }
            }
        }

        for word in rev_words.clone().into_iter() {
            let this_txt = String::new() + txt + &txt[..word.len() - 1];
            let matches = this_txt.match_indices(&word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((h, i % txt.len()));
                }
            }
            let matches = this_txt.rmatch_indices(&word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((h, i % txt.len()));
                }
            }
        }
    }

    let new = transpose(input_str[2..].iter().collect_vec());
    for (v, txt) in new.iter().enumerate() {
        for word in words.clone().into_iter() {
            let this_txt = txt;
            //let this_txt = String::new() + txt + &txt[..min(word.len() - 1, txt.len())];
            let matches = this_txt.match_indices(word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((i % txt.len(), v));
                }
            }
            let matches = this_txt.rmatch_indices(word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((i % txt.len(), v));
                }
            }
        }

        for word in rev_words.clone().into_iter() {
            let this_txt = txt;
            let matches = this_txt.match_indices(&word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((i % txt.len(), v));
                }
            }
            let matches = this_txt.rmatch_indices(&word).collect_vec();
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((i % txt.len(), v));
                }
            }
        }
    }
    inds.len().to_string()
}

fn run_all<'a>(input_str: Vec<String>, part: isize) -> String {
    let mut words = input_str[0][6..]
        .split(',')
        .map(|x| x.to_string())
        .collect_vec();
    let rev_words = input_str[0][6..]
        .split(',')
        .map(|x| x.chars().rev().collect::<String>())
        .collect_vec();
    words.extend(rev_words);

    let input_transposed = transpose(input_str[2..].iter().collect_vec());
    let mut inds = HashSet::new();

    for (h, txt) in input_str[2..].iter().enumerate() {
        for word in words.clone().into_iter() {
            let this_txt = if part == 3 {
                String::new() + txt + &txt[..word.len() - 1]
            } else {
                txt.to_string()
            };
            let mut matches = this_txt.match_indices(&word).collect_vec();
            matches.extend(this_txt.rmatch_indices(&word).collect_vec());
            for i_match in matches.iter() {
                for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                    inds.insert((h, i % txt.len()));
                }
            }
        }
    }
    if part == 3 {
        for (v, txt) in input_transposed.iter().enumerate() {
            for word in words.clone().into_iter() {
                let mut matches = txt.match_indices(&word).collect_vec();
                matches.extend(txt.rmatch_indices(&word).collect_vec());
                for i_match in matches.iter() {
                    for i in i_match.0..(i_match.0 + (i_match.1.len())) {
                        inds.insert((i % txt.len(), v));
                    }
                }
            }
        }
    }
    inds.len().to_string()
}

fn main() {
    let input_str = util::read_input(-1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input(-2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(-3);
    println!("{}", run_part3(input_str));

    let input_str = util::read_input(3);
    println!("{}", run_part3(input_str));

    let input_str = util::read_input(-2);
    println!("{}", run_all(input_str, 2));

    let input_str = util::read_input(2);
    println!("{}", run_all(input_str, 2));

    let input_str = util::read_input(-3);
    println!("{}", run_all(input_str, 3));

    let input_str = util::read_input(3);
    println!("{}", run_all(input_str, 3));
}
