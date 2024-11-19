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

use num_traits::{One, Pow, PrimInt, Zero};
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
            sum = sum + if self.0[idx] > rhs.0[idx] {
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
            sum = sum + if self.0[idx] > rhs.0[idx] {
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

    println!("{}, {}, {}", pt1.manhattan_dist(pt2), pt1.euclidean_dist_sq(pt2), pt1.euclidean_dist(pt2))
}