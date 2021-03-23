use std::collections::{BTreeMap, BinaryHeap, VecDeque};

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
