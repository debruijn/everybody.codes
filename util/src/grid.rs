// Brainstorm:
// Grid (default 2D but can be higher, or 1D)
// - Can create with known dimensions, and prefill with values
// - Can also keep track of grid in a sparse way
// - Size of grid can be readjusted
// Access: grid.get(tuple) -> BAM
// Access: grid.get_col(row) and vice versa -> how to extend to higher dims?
// Option to have a "curr_loc", so you can do grid.get_loc
// Option to iterate over grid
// Option to get neighbors from curr_loc or input loc
// - Outside grid: loop-around, default val, or no val
// - Diagonal or not?
// - Include self or not?
// - As tuple, array or vec?

// Util to construct grid from an input with #s and .s (and other chars) -> overwrite with nums?

// Also keep track of direction? Might want to make loc a separate struct

// Perhaps first create Point?
// - loc
// - dist
// -

use itertools::Itertools;
use num_traits::{One, Pow, PrimInt, ToPrimitive, Zero};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};
// TODO: could make utility impls for 2d and 3d situations, incl getting data quickly with x, y, z

pub trait Point1D: Default + PrimInt + Display + Zero + One + Mul {}

impl<T> Point1D for T where T: Default + PrimInt + Display + Zero + One + Mul {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point<T: Point1D, const N: usize>([T; N]);

impl<T: Point1D, const N: usize> Point<T, N> {
    pub fn new(coordinates: [T; N]) -> Self {
        Point(coordinates)
    }

    pub fn add_inplace(mut self, rhs: Self) {
        // TODO replace this and others with AddAssign and others
        let new = self + rhs;
        self.0 = new.0;
    }

    pub fn sub_inplace(mut self, rhs: Self) {
        let new = self - rhs;
        self.0 = new.0;
    }

    pub fn mul_inplace(mut self, rhs: Self) {
        let new = self * rhs;
        self.0 = new.0;
    }

    pub fn div_inplace(mut self, rhs: Self) {
        let new = self / rhs;
        self.0 = new.0;
    }

    pub fn mul_num(self, rhs: T) -> Self {
        let mut sum = self.0;
        for idx in 0..N {
            sum[idx] = sum[idx] * rhs;
        }
        Self(sum)
    }

    pub fn div_num(self, rhs: T) -> Self {
        let mut sum = self.0;
        for idx in 0..N {
            sum[idx] = sum[idx] / rhs;
        }
        Self(sum)
    }

    pub fn mul_num_inplace(mut self, rhs: T) {
        let new = self.mul_num(rhs);
        self.0 = new.0
    }

    pub fn div_num_inplace(mut self, rhs: T) {
        let new = self.div_num(rhs);
        self.0 = new.0
    }

    pub fn manhattan_dist(self, rhs: Self) -> T {
        let mut sum: T = T::zero();
        for idx in 0..N {
            sum = sum
                + if self.0[idx] > rhs.0[idx] {
                    self.0[idx] - rhs.0[idx]
                } else {
                    rhs.0[idx] - self.0[idx]
                };
        }
        sum
    }

    pub fn euclidean_dist_sq(self, rhs: Self) -> T {
        let mut sum: T = T::zero();
        for idx in 0..N {
            sum = sum
                + if self.0[idx] > rhs.0[idx] {
                    (self.0[idx] - rhs.0[idx]).pow(2)
                } else {
                    (rhs.0[idx] - self.0[idx]).pow(2)
                };
        }
        sum
    }

    pub fn euclidean_dist(self, rhs: Self) -> f64 {
        self.euclidean_dist_sq(rhs).to_f64().unwrap().pow(0.5)
    }

    pub fn gen_dist_sq(self, rhs: Self, pow: u32) -> T {
        let mut sum: T = T::zero();
        for idx in 0..N {
            sum = sum
                + if self.0[idx] > rhs.0[idx] {
                    (self.0[idx] - rhs.0[idx]).pow(pow)
                } else {
                    (rhs.0[idx] - self.0[idx]).pow(pow)
                };
        }
        sum
    }

    pub fn gen_dist(self, rhs: Self, pow: u32) -> f64 {
        self.gen_dist_sq(rhs, pow)
            .to_f64()
            .unwrap()
            .pow(1.0 / (pow.to_f64().unwrap()))
    }
}

impl<T: Point1D, const N: usize> Add for Point<T, N> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0..N {
            sum[idx] = sum[idx] + rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T: Point1D, const N: usize> Sub for Point<T, N> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0..N {
            sum[idx] = sum[idx] - rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T: Point1D, const N: usize> Div for Point<T, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0..N {
            sum[idx] = sum[idx] / rhs.0[idx];
        }
        Self(sum)
    }
}

impl<T: Point1D, const N: usize> Mul for Point<T, N> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut sum = self.0;
        for idx in 0..N {
            sum[idx] = sum[idx] * rhs.0[idx];
        }
        Self(sum)
    }
}

pub trait Gridlike: Default + PrimInt + Display + Zero + One + Mul {}

impl<T> Gridlike for T where T: Default + PrimInt + Display + Zero + One + Mul {}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T>
where
    T: Copy + From<u8> + Debug + PartialEq,
{
    pub fn new() -> Self {
        // Make a new empty Grid
        Grid(Vec::new())
    }

    pub fn from(vec_str: Vec<&str>) -> Self {
        // '8' -> 56 if T is u8, usize, etc
        let vec = vec_str
            .iter()
            .map(|x| x.bytes().map(|y| y.try_into().unwrap()).collect_vec())
            .collect_vec();
        Grid(vec)
    }

    pub fn from_map(vec_str: Vec<&str>, map: HashMap<char, T>) -> Self {
        let vec = vec_str
            .iter()
            .map(|x| x.chars().map(|y| map[&y]).collect_vec())
            .collect_vec();
        Grid(vec)
    }

    pub fn get_dims(&self) -> [usize; 2] {
        [self.0.len(), self.0[0].len()]
    }

    pub fn fill_lines(&mut self, fill: T) {
        let max_l = self.0.iter().map(|x| x.len()).max().unwrap();
        for row in self.0.iter_mut() {
            if row.len() < max_l {
                row.extend(vec![fill].repeat(max_l - row.len()))
            }
        }
    }

    pub fn set_elements(&mut self, elements: HashMap<Point<isize, 2>, T>) {
        for el in elements.iter() {
            self.0[el.0 .0[0] as usize][el.0 .0[1] as usize] = *el.1;
        }
    }

    pub fn set_pt(&mut self, el: T, pt: Point<isize, 2>) {
        self.0[pt.0[0] as usize][pt.0[1] as usize] = el;
    }

    pub fn set(&mut self, el: T, loc: (isize, isize)) {
        self.0[loc.0 as usize][loc.1 as usize] = el;
    }

    pub fn contains(&self, loc: Point<isize, 2>) -> bool {
        if (0 > loc.0[0]) || (loc.0[0] >= self.0.len() as isize) {
            false
        } else if (0 > loc.0[1]) || (loc.0[1] >= self.0[0].len() as isize) {
            false
        } else {
            true
        }
    }

    pub fn get(&self, loc: (isize, isize)) -> T {
        self.0[loc.0 as usize][loc.1 as usize]
    }

    pub fn get_pt(&self, pt: Point<isize, 2>) -> T {
        self.0[pt.0[0] as usize][pt.0[1] as usize]
    }

    pub fn get_elements(&self, pts: Vec<Point<isize, 2>>) -> Vec<T> {
        pts.iter().map(|pt| self.get_pt(*pt)).collect_vec()
    }

    // TODO: move neighbors to Point, to filter out usize vs isize
    pub fn get_neighbors(&self, pt: Point<isize, 2>) -> [T; 4] {
        let diffs = [Point([0, 1]), Point([1, 0]), Point([0, -1]), Point([-1, 0])];
        diffs
            .iter()
            .map(|x| self.get_pt(*x + pt))
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn get_neighbors_ok(&self, pt: Point<isize, 2>) -> Vec<(Point<isize, 2>, T)> {
        let diffs = [Point([0, 1]), Point([1, 0]), Point([0, -1]), Point([-1, 0])];
        let mut res = Vec::new();
        for diff in diffs.iter() {
            let this = pt + *diff;
            if self.contains(this) {
                res.push((this, self.get_pt(this)))
            }
        }
        res
    }

    pub fn get_neighbors_options(
        &self,
        pt: Point<isize, 2>,
        diag: bool,
        incl_pt: bool,
        wrap_around: bool,
    ) -> Vec<(Point<isize, 2>, T)> {
        let mut diffs = vec![Point([0, 1]), Point([1, 0]), Point([0, -1]), Point([-1, 0])];
        if incl_pt {
            diffs.push(Point([0, 0]));
        }
        if diag {
            diffs.extend(vec![
                Point([1, 1]),
                Point([1, -1]),
                Point([-1, -1]),
                Point([-1, 1]),
            ]);
        }

        let mut res = Vec::new();
        for diff in diffs {
            let this = pt + diff;
            if !wrap_around {
                if self.contains(this) {
                    res.push((this, self.get_pt(this)))
                }
            } else {
                let this = self.normalize(this);
                res.push((this, self.get_pt(this)))
            }
        }
        res
    }

    pub fn normalize(&self, pt: Point<isize, 2>) -> Point<isize, 2> {
        let dims = self.get_dims();
        Point(
            <Vec<isize> as TryInto<[isize; 2]>>::try_into(
                pt.0.into_iter()
                    .enumerate()
                    .map(|x| x.1.rem_euclid(dims[x.0] as isize))
                    .collect::<Vec<isize>>(),
            )
            .unwrap(),
        )
    }

    pub fn count(&self, key: T) -> usize {
        self.0
            .iter()
            .map(|x| x.iter().filter(|y| **y == key).count())
            .sum()
    }

    pub fn filter_key(&self, key: T) -> Vec<Point<isize, 2>> {
        self.0
            .iter()
            .enumerate()
            .map(|x| {
                x.1.iter()
                    .enumerate()
                    .filter(|y| *y.1 == key)
                    .map(move |y| Point([x.0 as isize, y.0 as isize]))
            })
            .flatten()
            .collect_vec()
    }
}

#[test]
fn math_operations() {
    // signed 2d  TODO Convert to actual test
    let pt1 = Point::new([6isize, 4]);
    let pt2 = Point::new([2isize, 2]);

    println!("{:?}", pt1 + pt2);
    println!("{:?}", pt1 - pt2);
    println!("{:?}", pt1 * pt2);
    println!("{:?}", pt1 / pt2);

    println!("{:?}", pt1.mul_num(2));
    println!("{:?}", pt1.div_num(2));

    pt1.add_inplace(pt2);
    println!("{:?}", pt1);
    pt1.mul_num_inplace(2);
    println!("{:?}", pt1);

    println!(
        "{}, {}, {}, {}, {}",
        pt1.manhattan_dist(pt2),
        pt1.euclidean_dist_sq(pt2),
        pt1.euclidean_dist(pt2),
        pt1.gen_dist_sq(pt2, 2),
        pt1.gen_dist(pt2, 1)
    );
    println!("{}", '8' as usize);

    let grid: Grid<u8> = Grid::from(vec!["abcd", "efgh"]);
    println!("{:?}", grid);
    let grid: Grid<char> = Grid::from(vec!["abcd", "efgh"]);
    println!("{:?}", grid);

    let map: HashMap<char, u8> = "abcdefghijkl"
        .chars()
        .map(|x| (x, x as u8 - 'a' as u8))
        .collect();
    let mut grid: Grid<u8> = Grid::from_map(vec!["abcd", "efgh", "ijkl"], map);
    println!("{:?}", grid);

    let map: HashMap<Point<isize, 2>, u8> = vec![(Point([0, 1]), 12u8), (Point([1, 1]), 15)]
        .into_iter()
        .collect();
    grid.set_elements(map);
    grid.set(4, (0, 0));
    grid.set_pt(0, Point([0, 3]));
    println!("{:?}", grid);
    println!("{}", grid.get((1, 1)));
    println!("{}", grid.get_pt(Point([1, 1])));

    println!("{:?}", grid.get_neighbors(Point([1, 1])));
    println!("{:?}", grid.get_neighbors_ok(Point([1, 1])));
    println!("{:?}", grid.get_neighbors_ok(Point([0, 1])));
    println!("{:?}", grid.get_neighbors_ok(Point([2, 3])));
    println!(
        "{:?}",
        grid.get_neighbors_options(Point([0, 1]), true, false, false)
    );
    println!(
        "{:?}",
        grid.get_neighbors_options(Point([0, 1]), true, true, false)
    );
    println!("{}, {}", (-1) % 3, (-1_isize).rem_euclid(3));
    println!(
        "{:?}",
        grid.get_neighbors_options(Point([0, 1]), false, false, true)
    );

    println!("{}, {:?}", grid.count(4), grid.filter_key(4));
    println!("{:?}", grid)
}
