//! This crate includes utility functions used by other workspace crates

use std::char;

use rand::{thread_rng, Rng};

/// convert (from..to) into (from, to)
pub fn expand_range<R: std::ops::RangeBounds<usize>>(range: R, max: usize) -> (usize, usize) {
    let from = match range.start_bound() {
        std::ops::Bound::Included(&from) => from,
        std::ops::Bound::Excluded(&from) => from + 1,
        std::ops::Bound::Unbounded => 0,
    };
    let to = match range.end_bound() {
        std::ops::Bound::Included(&end) => end + 1,
        std::ops::Bound::Excluded(&end) => end,
        std::ops::Bound::Unbounded => max,
    };
    (from, to)
}

/// return minimum `p` which applies `n <= 2 ^ p`
pub fn ceil_pow(n: usize) -> usize {
    n.next_power_of_two().trailing_zeros() as usize
}

fn shuffle<T>(v: &mut [T]) {
    let mut rng = thread_rng();
    for i in (1..v.len()).rev() {
        v.swap(rng.gen_range(0..=i), i);
    }
}

pub trait Shuffle<T> {
    fn shuffle(self) -> Self;
}

impl<T> Shuffle<T> for Vec<T> {
    fn shuffle(mut self) -> Self {
        shuffle(&mut self);
        self
    }
}

impl<T> Shuffle<T> for &mut [T] {
    fn shuffle(self) -> Self {
        shuffle(self);
        self
    }
}

pub trait IntoVec<T> {
    fn into(self) -> Vec<T>;
}

impl IntoVec<char> for &str {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl IntoVec<char> for &&str {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl IntoVec<char> for String {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl IntoVec<char> for &String {
    fn into(self) -> Vec<char> {
        self.chars().collect()
    }
}

impl<T: Clone> IntoVec<T> for &[T] {
    fn into(self) -> Vec<T> {
        self.to_vec()
    }
}

impl<T: Clone> IntoVec<T> for Vec<T> {
    fn into(self) -> Vec<T> {
        self
    }
}

impl<T: Clone> IntoVec<T> for &Vec<T> {
    fn into(self) -> Vec<T> {
        self.clone()
    }
}
/// Class that has additive identity element
pub trait Zero {
    /// The additive identity element
    fn zero() -> Self;
}

/// Class that has multiplicative identity element
pub trait One {
    /// The multiplicative identity element
    fn one() -> Self;
}

pub trait BoundedBelow {
    fn min_value() -> Self;
}

pub trait BoundedAbove {
    fn max_value() -> Self;
}

macro_rules! impl_integral {
    ($($ty:ty),*) => {
        $(
            impl Zero for $ty {
                #[inline]
                fn zero() -> Self {
                    0
                }
            }

            impl One for $ty {
                #[inline]
                fn one() -> Self {
                    1
                }
            }

            impl BoundedBelow for $ty {
                #[inline]
                fn min_value() -> Self {
                    Self::min_value()
                }
            }

            impl BoundedAbove for $ty {
                #[inline]
                fn max_value() -> Self {
                    Self::max_value()
                }
            }
        )*
    };
}

impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);

impl BoundedAbove for char {
    fn max_value() -> Self {
        std::u8::MAX as char
    }
}

impl BoundedBelow for char {
    fn min_value() -> Self {
        std::u8::MIN as char
    }
}
