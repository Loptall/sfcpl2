//! This crate includes utility functions used by other workspace crates

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
