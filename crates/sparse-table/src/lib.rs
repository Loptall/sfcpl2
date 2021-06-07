use std::{
    cmp::{max, min},
    marker::PhantomData,
    ops::RangeBounds,
};

use util::{math::ceil_pow, ExpandRange};

/// Binary operation which applies,
///
/// - Associativity (結合則): f(f(x, y), z) == f(x, f(y, z)),
/// - Idempotence (冪等性): f(x, x) == x,
///
/// For example...
/// - Max
/// - Min
/// - Lcm
/// - Gcd
pub trait Band {
    type S: Clone;
    fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S;
}

pub struct Max<S>(PhantomData<S>);

impl<S: Clone + Ord> Band for Max<S> {
    type S = S;

    fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S {
        max(x.clone(), y.clone())
    }
}

pub struct Min<S>(PhantomData<S>);

impl<S: Clone + Ord> Band for Min<S> {
    type S = S;

    fn binary_operation(x: &Self::S, y: &Self::S) -> Self::S {
        min(x.clone(), y.clone())
    }
}

/// Immutable Data Structure which can answer range query in `O(1)`
pub struct SparseTable<B: Band> {
    inner: Vec<Vec<B::S>>,
}

impl<B: Band> From<&[B::S]> for SparseTable<B> {
    fn from(v: &[B::S]) -> Self {
        Self::new(v)
    }
}

impl<B: Band> SparseTable<B> {
    pub fn new(v: &[B::S]) -> Self {
        let n = v.len();
        let p = ceil_pow(n);
        let mut inner = vec![Vec::new(); p];
        inner[0] = v.clone().to_vec();

        for i in 1..p {
            let mut append = Vec::new();
            for j in 0..n - (1 << i) + 1 {
                let t = B::binary_operation(&inner[i - 1][j], &inner[i - 1][j + (1 << (i - 1))]);
                append.push(t);
            }
            // dbg!(&append);
            inner[i] = append;
        }

        Self { inner }
    }

    pub fn len(&self) -> usize {
        self.raw().len()
    }

    pub fn raw(&self) -> &[B::S] {
        &self.inner[0]
    }

    fn fold_sub(&self, from: usize, to: usize) -> B::S {
        assert!(from < to);
        let d = to - from;
        if d == 1 {
            self.raw()[from].clone()
        } else {
            let p = ceil_pow(d) - 1;
            B::binary_operation(&self.inner[p][from], &self.inner[p][to - (1 << p)])
        }
    }

    pub fn fold<R>(&self, range: R) -> B::S
    where
        R: RangeBounds<usize>,
    {
        let (from, to) = ExpandRange::expand_range(range, 0, self.len());
        self.fold_sub(from, to)
    }
}

#[cfg(test)]
mod tests {
    use rand::{thread_rng, Rng};

    use super::{Min, SparseTable};

    #[test]
    fn basic() {
        let v = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9, 3];
        let st = SparseTable::<Min<usize>>::new(&v);
        // dbg!(st.inner);

        // assert_eq!(st.fold(0, 0), 3); // panic: empty range
        assert_eq!(st.fold(0..1), 3);
        assert_eq!(st.fold(0..2), 1);
        assert_eq!(st.fold(0..3), 1);
        assert_eq!(st.fold(4..7), 2);
        assert_eq!(st.fold(8..10), 3);
    }

    #[test]
    fn large_random() {
        let mut rng = thread_rng();
        let mut v = Vec::new();
        for _ in 0..200000 {
            v.push(rng.gen::<usize>());
        }

        let n = v.len();
        let sparse_table = SparseTable::<Min<_>>::new(&v);
        for _ in 0..100 {
            let mut a = rng.gen_range(0, n + 1);
            let mut b = rng.gen_range(0, n + 1);
            if a == b {
                continue;
            }
            if b < a {
                std::mem::swap(&mut a, &mut b);
            }

            assert_eq!(sparse_table.fold(a..b), (a..b).map(|x| v[x]).min().unwrap());
        }
    }
}
