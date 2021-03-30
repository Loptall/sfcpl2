//! This crate includes utility functions used by other workspace crates

use std::collections::{BTreeMap, BinaryHeap, VecDeque};

use rand::{thread_rng, Rng};

pub mod math;
pub mod timer;
pub mod traits;

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

pub trait UniqueCount<T> {
    /// count the number of each value in self
    fn unique_count(&self) -> BTreeMap<T, usize>;
}

impl<T: Ord + Clone> UniqueCount<T> for &[T] {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &[&T] {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for Vec<T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for Vec<&T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &Vec<T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }

        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &Vec<&T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for VecDeque<T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for VecDeque<&T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &VecDeque<T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &VecDeque<&T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for BinaryHeap<T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for BinaryHeap<&T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &BinaryHeap<T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl<T: Ord + Clone> UniqueCount<T> for &BinaryHeap<&T> {
    fn unique_count(&self) -> BTreeMap<T, usize> {
        let mut res = BTreeMap::new();
        for &ele in self.iter() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl UniqueCount<char> for &str {
    fn unique_count(&self) -> BTreeMap<char, usize> {
        let mut res = BTreeMap::new();
        for ele in self.chars() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
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
