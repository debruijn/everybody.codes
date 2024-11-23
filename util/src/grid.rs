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

use itertools::{Itertools, MinMaxResult};
use num_traits::{Bounded, Euclid, One, Pow, PrimInt, ToPrimitive, Zero};
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Sub},
};
// TODO: could make utility impls for 2d and 3d situations, incl getting data quickly with x, y, z

pub trait Point1D: Debug + Default + PrimInt + Display + Zero + One + Mul + Neg + From<i8> {}

impl<T> Point1D for T where T: Debug + Default + PrimInt + Display + Zero + One + Mul + Neg + From<i8> {}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point<T: Point1D, const N: usize>(pub [T; N]);

impl<T: Point1D, const N: usize> Point<T, N> {
    pub fn new(coordinates: [T; N]) -> Self {
        Point(coordinates)
    }

    pub fn from_2d(pt: Point<T, 2>) -> Self {
        if N == 2 {
            Self::new(vec!(pt.0[0], pt.0[1]).try_into().unwrap())
        } else if N < 2 {
            Self::new(vec!(pt.0[0]).try_into().unwrap())
        } else {
            let mut coords = vec!(pt.0[0], pt.0[1]);
            coords.extend(vec!(T::zero(); N-2).iter());
            Self::new(coords.try_into().unwrap())
        }
    }

    pub fn get(self) -> [T; N] {
        self.0
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

    pub fn zero() -> Self {
        Self([T::zero(); N])
    }

    pub fn axis(ax: usize) -> Self {
        let mut axis = Self::zero();
        axis.0[ax] = T::one();
        axis
    }

    pub fn naxis(ax: usize) -> Self {
        let mut axis = Self::zero();
        axis.0[ax] = T::one();
        axis.neg()
    }

    pub fn dirs() -> Vec<Self> {
        let mut dir_vec = Vec::new();
        for i in 0..N {
            dir_vec.push(Self::axis(i));
            dir_vec.push(Self::naxis(i));
        }
        dir_vec
    }

    pub fn diag_dirs() -> Vec<Self> {
        let dirs_by_dir = Self::dirs_by_dir();
        let mut res = Vec::new();
        for this_combo in dirs_by_dir.iter().multi_cartesian_product() {
            let mut this_pt = Point::<T,N>::zero();
            for pt in this_combo.into_iter() {
                this_pt = this_pt + *pt;
            }
            res.push(this_pt);
        }
        res
    }

    pub fn get_zero(self) -> Self {
        Self([T::zero(); N])
    }

    pub fn get_axis(self, ax: usize) -> Self {
        Self::axis(ax)
    }

    pub fn get_naxis(self, ax: usize) -> Self {
        Self::naxis(ax)
    }

    pub fn get_dirs(self) -> Vec<Self> {
        Self::dirs()
    }

    fn dirs_by_dir() -> Vec<[Self;3]> {
        let mut dir_vec = Vec::new();
        for i in 0..N {
            dir_vec.push([Self::axis(i), Self::zero(), Self::naxis(i)]);
        }
        dir_vec
    }

    pub fn get_diag_dirs(self) -> Vec<Self> {
        Self::diag_dirs()
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

impl<T: Point1D, const N: usize> Neg for Point<T, N> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut new = Self::zero();
        for idx in 0..N {
            new.0[idx] = self.0[idx].mul((-1).try_into().unwrap());
        }
        new
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

    pub fn from_string(vec_str: Vec<String>) -> Self {
        // '8' -> 56 if T is u8, usize, etc
        let vec = vec_str
            .iter()
            .map(|x| x.bytes().map(|y| y.try_into().unwrap()).collect_vec())
            .collect_vec();
        Grid(vec)
    }

    pub fn from_str(vec_str: Vec<&str>) -> Self {
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

    pub fn filter_first(&self, key: T) -> Point<isize, 2> {
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
            .next().unwrap()
    }

    pub fn filter_keys(&self, keys: Vec<T>) -> Vec<Point<isize, 2>> {
        self.0
            .iter()
            .enumerate()
            .map(|x| {
                x.1.iter()
                    .enumerate()
                    .filter(|y| keys.contains(y.1))
                    .map(move |y| Point([x.0 as isize, y.0 as isize]))
            })
            .flatten()
            .collect_vec()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridSparse2D<T, U>(HashMap<Point<U, 2>, T>)
where
    U: Point1D + Hash + Debug + TryFrom<usize> + Bounded;

impl<T, U, E> GridSparse2D<T, U>
where
    T: Copy + From<u8> + Debug + PartialEq,
    U: Point1D + Hash + TryFrom<usize, Error = E> + Debug + Neg<Output = U> + Euclid,
    E: Debug,
{
    pub fn new() -> Self {
        // Make a new empty Grid
        GridSparse2D(HashMap::new())
    }

    pub fn from_string(vec_str: Vec<String>, ign: Vec<u8>) -> Self {
        // '8' -> 56 if T is u8, usize, etc
        let mut this_map: HashMap<Point<U, 2>, T> = HashMap::new();
        for (x, row) in vec_str.iter().enumerate() {
            for (y, el) in row.bytes().enumerate().filter(|y| !ign.contains(&y.1)) {
                this_map.insert(
                    Point([x.try_into().unwrap(), y.try_into().unwrap()]),
                    el.try_into().unwrap(),
                );
            }
        }
        GridSparse2D(this_map)
    }

    pub fn from_str(vec_str: Vec<&str>, ign: Vec<u8>) -> Self {
        // '8' -> 56 if T is u8, usize, etc
        let mut this_map: HashMap<Point<U, 2>, T> = HashMap::new();
        for (x, row) in vec_str.iter().enumerate() {
            for (y, el) in row.bytes().enumerate().filter(|y| !ign.contains(&y.1)) {
                this_map.insert(
                    Point([x.try_into().unwrap(), y.try_into().unwrap()]),
                    el.try_into().unwrap(),
                );
            }
        }
        GridSparse2D(this_map)
    }

    pub fn from_map(vec_str: Vec<&str>, map: HashMap<char, T>) -> Self {
        let mut this_map: HashMap<Point<U, 2>, T> = HashMap::new();
        for (x, row) in vec_str.iter().enumerate() {
            for (y, el) in row.chars().enumerate() {
                if map.keys().contains(&el) {
                    this_map.insert(
                        Point([x.try_into().unwrap(), y.try_into().unwrap()]),
                        map[&el],
                    );
                }
            }
        }
        GridSparse2D(this_map)
    }

    pub fn get_dims(&self) -> [U; 2] {
        self.get_bounds()
            .iter()
            .map(|x| x[0] - x[1] + U::one())
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn get_bounds(&self) -> [[U; 2]; 2] {
        let x_minmax: [U; 2] = match self.0.keys().map(|x| x.0[0]).minmax() {
            MinMaxResult::NoElements => [U::min_value(), U::max_value()], // Revert order on purpose
            MinMaxResult::OneElement(a) => [a, a],
            MinMaxResult::MinMax(a, b) => [a, b],
        }; //take min max
        let y_minmax: [U; 2] = match self.0.keys().map(|x| x.0[1]).minmax() {
            MinMaxResult::NoElements => [U::min_value(), U::max_value()], // Revert order on purpose
            MinMaxResult::OneElement(a) => [a, a],
            MinMaxResult::MinMax(a, b) => [a, b],
        };
        [x_minmax, y_minmax]
    }

    pub fn fill_lines(&mut self, _fill: T) {
        // TODO: now does nothing; could make it to fill it (which goes against use of sparse grid)
    }

    pub fn set_elements(&mut self, elements: HashMap<Point<U, 2>, T>) {
        for (key, value) in elements.iter() {
            self.0.insert(*key, *value);
        }
    }

    pub fn set_pt(&mut self, el: T, pt: Point<U, 2>) {
        self.0.insert(pt, el);
    }

    pub fn set(&mut self, el: T, loc: (U, U)) {
        self.0.insert(Point(loc.into()), el);
    }

    pub fn contains(&self, pt: Point<U, 2>) -> bool {
        self.0.contains_key(&pt)
    }

    pub fn get(&self, loc: (U, U)) -> T {
        self.0[&Point(loc.into())]
    }

    pub fn get_pt(&self, pt: Point<U, 2>) -> T {
        self.0[&pt]
    }

    pub fn get_elements(&self, pts: Vec<Point<U, 2>>) -> Vec<T> {
        pts.iter().map(|pt| self.get_pt(*pt)).collect_vec()
    }

    // // TODO: move neighbors to Point, to filter out usize vs isize
    pub fn get_neighbors(&self, pt: Point<U, 2>) -> [T; 4] {
        let diffs: [Point<U, 2>; 4] = [
            Point([U::zero(), U::one()]),
            Point([U::one(), U::zero()]),
            Point([U::zero(), -U::one()]),
            Point([-U::one(), U::zero()]),
        ];
        diffs
            .iter()
            .map(|x| self.get_pt(*x + pt))
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn get_neighbors_ok(&self, pt: Point<U, 2>) -> Vec<(Point<U, 2>, T)> {
        let diffs = [
            Point([U::zero(), U::one()]),
            Point([U::one(), U::zero()]),
            Point([U::zero(), -U::one()]),
            Point([-U::one(), U::zero()]),
        ];
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
        pt: Point<U, 2>,
        diag: bool,
        incl_pt: bool,
        _wrap_around: bool,
    ) -> Vec<(Point<U, 2>, T)> {
        let mut diffs = vec![
            Point([U::zero(), U::one()]),
            Point([U::one(), U::zero()]),
            Point([U::zero(), -U::one()]),
            Point([-U::one(), U::zero()]),
        ];
        if incl_pt {
            diffs.push(Point([U::zero(), U::zero()]));
        }
        if diag {
            diffs.extend(vec![
                Point([U::one(), U::one()]),
                Point([U::one(), -U::one()]),
                Point([-U::one(), -U::one()]),
                Point([-U::one(), U::one()]),
            ]);
        }

        let mut res = Vec::new();
        for diff in diffs {
            let this = pt + diff;
            if self.contains(this) {
                res.push((this, self.get_pt(this)))
            }
        }
        res
    }

    pub fn normalize(&self, pt: Point<U, 2>) -> Point<U, 2> {
        let dims = self.get_dims();
        let bounds = self.get_bounds();
        Point(
            <Vec<U> as TryInto<[U; 2]>>::try_into(
                pt.0.into_iter()
                    .enumerate()
                    .map(|x| (x.1 - bounds[x.0][0]).rem_euclid(&dims[x.0]) + bounds[x.0][0])
                    .collect::<Vec<U>>(),
            )
            .unwrap(),
        )
    }

    pub fn count(&self, key: T) -> usize {
        self.0.iter().filter(|x| *x.1 == key).count()
    }

    pub fn filter_key(&self, key: T) -> Vec<Point<U, 2>> {
        self.0
            .iter()
            .filter(|x| *x.1 == key)
            .map(|x| *x.0)
            .collect_vec()
    }

    pub fn filter_first(&self, key: T) -> Point<U, 2> {
        *self.0
            .iter()
            .filter(|x| *x.1 == key)
            .next().unwrap().0
    }

    pub fn filter_keys(&self, keys: Vec<T>) -> Vec<Point<U, 2>> {
        self.0
            .iter()
            .filter(|x| keys.contains(x.1))
            .map(|x| *x.0)
            .collect_vec()
    }
}


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GridSparse<T, const N: usize, U>(HashMap<Point<U, N>, T>)
where
    U: Point1D + Hash + Debug + TryFrom<usize> + Bounded;

impl<T, const N: usize, U, E> GridSparse<T, N, U>
where
    T: Copy + From<u8> + Debug + PartialEq,
    U: Point1D + Hash + TryFrom<usize, Error = E> + Debug + Neg<Output = U> + Euclid,
    E: Debug,
{
    pub fn new() -> Self {
        // Make a new empty Grid
        GridSparse(HashMap::new())
    }

    pub fn from_string(vec_str: Vec<String>, ign: Vec<u8>) -> Self {
        // '8' -> 56 if T is u8, usize, etc
        let mut this_map: HashMap<Point<U, N>, T> = HashMap::new();
        for (x, row) in vec_str.iter().enumerate() {
            for (y, el) in row.bytes().enumerate().filter(|y| !ign.contains(&y.1)) {
                this_map.insert(
                    Point::from_2d(Point([x.try_into().unwrap(), y.try_into().unwrap()])),
                    el.try_into().unwrap(),
                );
            }
        }
        GridSparse(this_map)
    }

    pub fn from_str(vec_str: Vec<&str>, ign: Vec<u8>) -> Self {
        // '8' -> 56 if T is u8, usize, etc
        let mut this_map: HashMap<Point<U, N>, T> = HashMap::new();
        for (x, row) in vec_str.iter().enumerate() {
            for (y, el) in row.bytes().enumerate().filter(|y| !ign.contains(&y.1)) {
                this_map.insert(
                    Point::from_2d(Point([x.try_into().unwrap(), y.try_into().unwrap()])),
                    el.try_into().unwrap(),
                );
            }
        }
        GridSparse(this_map)
    }

    pub fn from_map(vec_str: Vec<&str>, map: HashMap<char, T>) -> Self {
        let mut this_map: HashMap<Point<U, N>, T> = HashMap::new();
        for (x, row) in vec_str.iter().enumerate() {
            for (y, el) in row.chars().enumerate() {
                if map.keys().contains(&el) {
                    this_map.insert(
                        Point::from_2d(Point([x.try_into().unwrap(), y.try_into().unwrap()])),
                        map[&el],
                    );
                }
            }
        }
        GridSparse(this_map)
    }

    pub fn get_dims(&self) -> [U; N] {
        self.get_bounds()
            .iter()
            .map(|x| x[0] - x[1] + U::one())
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn get_bounds(&self) -> [[U; 2]; N] {
        let mut res = Vec::new();
        for dim in 0..N {
            let dim_minmax: [U; 2] = match self.0.keys().map(|x| x.0[dim]).minmax() {
                MinMaxResult::NoElements => [U::min_value(), U::max_value()], // Revert order on purpose
                MinMaxResult::OneElement(a) => [a, a],
                MinMaxResult::MinMax(a, b) => [a, b],
            };
            res.push(dim_minmax);
        }
        res.try_into().unwrap()
    }

    pub fn fill_lines(&mut self, _fill: T) {
        // TODO: now does nothing; could make it to fill it (which goes against use of sparse grid)
    }

    pub fn set_elements(&mut self, elements: HashMap<Point<U, N>, T>) {
        for (key, value) in elements.iter() {
            self.0.insert(*key, *value);
        }
    }

    pub fn set_pt(&mut self, el: T, pt: Point<U, N>) {
        self.0.insert(pt, el);
    }

    pub fn set(&mut self, el: T, loc: [U; N]) {
        self.0.insert(Point(loc.into()), el);
    }

    pub fn contains(&self, pt: Point<U, N>) -> bool {
        self.0.contains_key(&pt)
    }

    pub fn get(&self, loc: [U; N]) -> T {
        self.0[&Point(loc.into())]
    }

    pub fn get_pt(&self, pt: Point<U, N>) -> T {
        self.0[&pt]
    }

    pub fn get_elements(&self, pts: Vec<Point<U, N>>) -> Vec<T> {
        pts.iter().map(|pt| self.get_pt(*pt)).collect_vec()
    }

    // // TODO: move neighbors to Point, to filter out usize vs isize
    pub fn get_neighbors(&self, pt: Point<U, N>) -> Vec<T> {
        let diffs = Point::<U, N>::dirs();
        diffs
            .iter()
            .map(|x| self.get_pt(*x + pt))
            .collect_vec()
            .try_into()
            .unwrap()
    }

    pub fn get_neighbors_ok(&self, pt: Point<U, N>) -> Vec<(Point<U, N>, T)> {
        let diffs = Point::<U, N>::dirs();

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
        pt: Point<U, N>,
        diag: bool,
        incl_pt: bool,
        _wrap_around: bool,
    ) -> Vec<(Point<U, N>, T)> {

        let diffs: Vec<Point<U,N>> = if diag {
            let mut diffs = Point::<U, N>::diag_dirs();
            if !incl_pt {
                let zero_ind = diffs.iter().position(|x| *x == Point::<U, N>::zero()).unwrap();
                diffs.swap_remove(zero_ind);
            }
            diffs
        } else {
            let mut diffs = Point::<U, N>::dirs();
            if incl_pt {
                diffs.push(Point::<U, N>::zero());
            }
            diffs
        };

        let mut res = Vec::new();
        for diff in diffs {
            let this = pt + diff;
            if self.contains(this) {
                res.push((this, self.get_pt(this)))
            }
        }
        res
    }

    pub fn normalize(&self, pt: Point<U, N>) -> Point<U, N> {
        let dims = self.get_dims();
        let bounds = self.get_bounds();
        Point(
            <Vec<U> as TryInto<[U; N]>>::try_into(
                pt.0.into_iter()
                    .enumerate()
                    .map(|x| (x.1 - bounds[x.0][0]).rem_euclid(&dims[x.0]) + bounds[x.0][0])
                    .collect::<Vec<U>>(),
            )
                .unwrap(),
        )
    }

    pub fn count(&self, key: T) -> usize {
        self.0.iter().filter(|x| *x.1 == key).count()
    }

    pub fn filter_key(&self, key: T) -> Vec<Point<U, N>> {
        self.0
            .iter()
            .filter(|x| *x.1 == key)
            .map(|x| *x.0)
            .collect_vec()
    }

    pub fn filter_first(&self, key: T) -> Point<U, N> {
        *self.0
            .iter()
            .filter(|x| *x.1 == key)
            .next().unwrap().0
    }

    pub fn filter_keys(&self, keys: Vec<T>) -> Vec<Point<U, N>> {
        self.0
            .iter()
            .filter(|x| keys.contains(x.1))
            .map(|x| *x.0)
            .collect_vec()
    }
}


#[test]
fn try_stuff_out() {
    // TODO Convert to actual test
    type GridSparse2D<T, U> = GridSparse<T, 2, U>;

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

    println!(
        "{:?}, {:?}",
        Point::<isize, 2>::axis(1),
        Point::<i8, 3>::naxis(0)
    );
    println!(
        "{:?}, {:?}",
        Point::<isize, 2>::zero(),
        Point::<i8, 3>::zero()
    );
    println!("{:?}", Point::<i16, 4>::dirs());
    println!("{:?}", pt1.get_dirs());
    println!("{:?}", Point::<i128, 3>::diag_dirs());

    let grid: Grid<u8> = Grid::from_str(vec!["abcd", "efgh"]);
    println!("{:?}", grid);
    let grid: Grid<char> = Grid::from_str(vec!["abcd", "efgh"]);
    println!("{:?}", grid);

    let grid: GridSparse2D<u8, isize> = GridSparse2D::from_str(vec!["abcd", "efgh"], vec![]);
    println!("{:?}", grid);
    let grid: GridSparse2D<char, isize> = GridSparse2D::from_str(vec!["abcd", "efgh"], vec![]);
    println!("{:?}", grid);

    let map: HashMap<char, u8> = "abcdefghijkl"
        .chars()
        .map(|x| (x, x as u8 - 'a' as u8))
        .collect();
    let grid1: Grid<u8> = Grid::from_map(vec!["abcd", "efgh", "ijkl"], map.clone()); // TODO make input &
    println!("{:?}", grid1);
    let grid2: GridSparse2D<u8, isize> = GridSparse2D::from_map(vec!["abcd", "efgh", "ijkl"], map);
    println!("{:?}", grid2);

    let mut grid = grid1;
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
    println!("{:?}", grid);

    let mut grid = grid2;
    let map: HashMap<Point<isize, 2>, u8> = vec![(Point([0, 1]), 12), (Point([1, 1]), 15)]
        .into_iter()
        .collect();
    grid.set_elements(map);
    // grid.set(4, (0, 0));
    grid.set_pt(0, Point([0, 3]));
    println!("{:?}", grid);
    // println!("{}", grid.get((1, 1)));
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
    println!("{:?}", grid);
}
