use everybody_codes_util as util;
use itertools::Itertools;
use util::grid::{GridSparse2D, Point};

type Pt = Point<isize, 2>;


fn find_closest(points: &Vec<&Pt>, constellation: &Vec<Pt>) -> (Pt, isize){
    // Find the point with the lowest distance to a point in the constellation
    points
        .iter()
        .filter(|x| !constellation.contains(x))
        .map(|x| {
            (
                **x,
                constellation
                    .iter()
                    .map(|y| y.manhattan_dist(**x))
                    .min()
                    .unwrap(),
            )
        })
        .min_by_key(|x| x.1)
        .unwrap()
}


fn run_part12(input_str: Vec<String>, _example: bool) -> String {
    let grid: GridSparse2D<u8, isize> = GridSparse2D::from_string(input_str, vec![b'.']);
    let points: Vec<&Pt> = grid.keys().collect_vec();
    let mut constellation: Vec<Pt> = vec![*points[0]];

    let mut constellation_len = points.len() as isize;
    while constellation.len() < points.len() {
        let (pt, distance) = find_closest(&points, &constellation);
        constellation.push(pt);
        constellation_len += distance;
    }

    constellation_len.to_string()
}

fn run_part3(input_str: Vec<String>, _example: bool) -> String {
    let grid: GridSparse2D<u8, isize> = GridSparse2D::from_string(input_str, vec![b'.']);
    let mut points: Vec<&Pt> = grid.keys().collect_vec();

    let mut score = Vec::new();
    while points.len() > 0 {
        let mut constellation: Vec<Pt> = vec![*points[0]];
        points.remove(0);
        let mut constellation_len = 0;
        while points.len() > 0 {
            let (pt, distance) = find_closest(&points, &constellation);
            if distance < 6 {
                constellation.push(pt);
                constellation_len += distance;
                points.remove(points.iter().position(|x| **x == pt).unwrap());
            } else {
                break;
            }
        }
        score.push(constellation_len + constellation.len() as isize);
    }

    score.iter().k_largest(3).product::<isize>().to_string()
}

fn main() {
    // Part 1: example and actual
    println!("Part 1");
    util::run(run_part12, -1); // Included because first call is slower for allocation reasons
    util::run(run_part12, 1);
    println!("Example: {}", util::run(run_part12, -1));
    println!("Actual: {}\n", util::run(run_part12, 1));

    // Part 2: example and actual
    println!("Part 2");
    println!("Actual: {}\n", util::run(run_part12, 2));

    // Part 3: example and actual
    println!("Part 3");
    println!("Example: {}", util::run(run_part3, -3));
    println!("Actual: {}\n", util::run(run_part3, 3));
}
