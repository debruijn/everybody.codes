use itertools::Itertools;

mod util;

fn run_part1<'a>(input_str: Vec<String>) -> String {
    let mut nr_potions = 0;
    for creature in input_str[0].chars() {
        let increase = match creature {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            _ => 0,
        };
        nr_potions += increase
    }
    nr_potions.to_string()
}

fn run_part2<'a>(input_str: Vec<String>) -> String {
    let mut nr_potions = 0;
    for mut battle in &input_str[0].chars().chunks(2) {
        if !battle.any(|x| x == 'x') {
            nr_potions += 2
        }
    }
    for mut battle in &input_str[0].chars().chunks(2) {

        for creature in battle {
            let increase = match creature {
                'A' => 0,
                'B' => 1,
                'C' => 3,
                'D' => 5,
                _ => 0,
            };
            nr_potions += increase;
        }
    }
    nr_potions.to_string()
}

fn run_part3<'a>(input_str: Vec<String>) -> String {
    let mut nr_potions = 0;
    for mut battle in &input_str[0].chars().chunks(3) {
        let sum_x = battle.map(|x| if x == 'x' {0} else {1}).sum();
        let increase = match sum_x {
            2 => 2,
            3 => 6,
            _ => 0
        };
        nr_potions += increase;
    }
    for mut battle in &input_str[0].chars().chunks(3) {

        for creature in battle {
            let increase = match creature {
                'A' => 0,
                'B' => 1,
                'C' => 3,
                'D' => 5,
                _ => 0,
            };
            nr_potions += increase;
        }
    }
    nr_potions.to_string()
}



fn main() {
    let input_str = util::read_input(-1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input( 1);
    println!("{}", run_part1(input_str));

    let input_str = util::read_input( -2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(2);
    println!("{}", run_part2(input_str));

    let input_str = util::read_input(-3);
    println!("{}", run_part3(input_str));

    let input_str = util::read_input( 3);
    println!("{}", run_part3(input_str));
}
