//! This crate includes utility functions used by other workspace crates

use std::collections::{BTreeMap, BinaryHeap, VecDeque};

use rand::{thread_rng, Rng};

pub mod math;
pub mod traits;

/// convert (from..to) into (from, to)
pub trait ExpandRange: Sized {
    fn expand_range<R: std::ops::RangeBounds<Self>>(range: R, min: Self, max: Self) -> (Self, Self);
}

macro_rules! impl_expand_range {
    ($t:ty) => {
        impl ExpandRange for $t {
            /// convert (from..to) into (from, to)
            fn expand_range<R: std::ops::RangeBounds<Self>>(range: R, min: Self, max: Self) -> (Self, Self) {
                let from = match range.start_bound() {
                    std::ops::Bound::Included(&from) => from,
                    std::ops::Bound::Excluded(&from) => from + 1,
                    std::ops::Bound::Unbounded => min,
                };
                let to = match range.end_bound() {
                    std::ops::Bound::Included(&end) => end + 1,
                    std::ops::Bound::Excluded(&end) => end,
                    std::ops::Bound::Unbounded => max,
                };
                (from, to)
            }
        }
    };
}

impl_expand_range!(usize);
impl_expand_range!(u8);
impl_expand_range!(u16);
impl_expand_range!(u32);
impl_expand_range!(u64);
impl_expand_range!(u128);
impl_expand_range!(i8);
impl_expand_range!(i16);
impl_expand_range!(i32);
impl_expand_range!(i64);
impl_expand_range!(i128);
// impl_expand_range!(char); neeeded?

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

impl UniqueCount<char> for String {
    fn unique_count(&self) -> BTreeMap<char, usize> {
        let mut res = BTreeMap::new();
        for ele in self.chars() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

impl UniqueCount<char> for &String {
    fn unique_count(&self) -> BTreeMap<char, usize> {
        let mut res = BTreeMap::new();
        for ele in self.chars() {
            *res.entry(ele.clone()).or_insert(0) += 1;
        }
        res
    }
}

pub trait RunLengthEncoding<T> {
    fn rle(&self) -> Vec<(T, usize)>;
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &[T] {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &[&T] {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for Vec<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for Vec<&T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &Vec<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &Vec<&T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for VecDeque<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for VecDeque<&T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &VecDeque<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &VecDeque<&T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for BinaryHeap<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for BinaryHeap<&T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &BinaryHeap<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &BinaryHeap<&T> {
    fn rle(&self) -> Vec<(T, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for &e in self.iter() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap().clone(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl RunLengthEncoding<char> for &str {
    fn rle(&self) -> Vec<(char, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.chars() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl RunLengthEncoding<char> for String {
    fn rle(&self) -> Vec<(char, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.chars() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

impl RunLengthEncoding<char> for &String {
    fn rle(&self) -> Vec<(char, usize)> {
        let mut res = Vec::new();
        let mut count = 0;
        let mut last = None;
        for e in self.chars() {
            match last {
                None => {
                    last = Some(e);
                    count += 1;
                }
                Some(old) if old == e => {
                    count += 1;
                }
                Some(_) => {
                    res.push((last.unwrap(), count));
                    last = Some(e);
                    count = 1;
                }
            }
        }
        res.push((last.unwrap().clone(), count));

        res
    }
}

// fn shuffle<T>(v: &mut [T]) {
//     let mut rng = thread_rng();
//     for i in (1..v.len()).rev() {
//         v.swap(rng.gen_range(0..=i), i);
//     }
// }

pub trait Shuffle: Sized {
    fn shuffle_with<R: Rng>(&mut self, rng: &mut R);
    fn shuffled_with<R: Rng>(self, rng: &mut R) -> Self {
        let mut res = self;
        res.shuffle_with(rng);
        res
    }
    fn shuffle(&mut self) {
        self.shuffle_with(&mut thread_rng());
    }
    fn shuffled(self) -> Self {
        self.shuffled_with(&mut thread_rng())
    }
}

impl<T> Shuffle for Vec<T> {
    fn shuffle_with<R: Rng>(&mut self, rng: &mut R) {
        for i in (1..self.len()).rev() {
            self.swap(rng.gen_range(0..=i), i);
        }
    }
}

impl<T> Shuffle for VecDeque<T> {
    fn shuffle_with<R: Rng>(&mut self, rng: &mut R) {
        for i in (1..self.len()).rev() {
            self.swap(rng.gen_range(0..=i), i);
        }
    }
}

impl Shuffle for String {
    fn shuffle_with<R: Rng>(&mut self, rng: &mut R) {
        let mut s = self.chars().collect::<Vec<_>>();
        for i in (1..self.len()).rev() {
            s.swap(rng.gen_range(0..=i), i);
        }
        *self = s.into_iter().collect();
    }
}

#[cfg(test)]
mod tests {
    use super::RunLengthEncoding as _;
    #[test]
    fn test_rle() {
        let v = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(v.rle(), vec![(1, 1), (2, 2), (3, 3)]);
    }
}
