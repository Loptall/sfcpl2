use std::{cmp::min, marker::PhantomData, ops::RangeBounds};

use rand::{thread_rng, Rng};

const MOD: u64 = (1 << 61) - 1;

pub trait Base {
    fn base() -> u64;
}

#[derive(Debug)]
struct RandomBase;

impl Base for RandomBase {
    fn base() -> u64 {
        thread_rng().gen_range(2..=MOD)
    }
}

#[derive(Debug, Clone)]
pub struct RollingHash<B, T> {
    phantom: PhantomData<B>,
    raw: Vec<T>,
    pow_of_base: Vec<u64>,
    hashed: Vec<u64>,
}

impl<B: Base, S: AsRef<str>> From<S> for RollingHash<B, char> {
    fn from(s: S) -> Self {
        Self::from_str(s)
    }
}

fn expand_range<R: RangeBounds<usize>>(range: R, max: usize) -> (usize, usize) {
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

fn powering(base: u64, n: usize) -> Vec<u64> {
    let mut pow_of_base = vec![1];
    (0..n).fold(1, |prd, _| {
        let next = prd * base % MOD;
        pow_of_base.push(next);
        next
    });
    pow_of_base
}

fn rolling(s: &[u128], base: u64) -> Vec<u64> {
    let mut hashed = vec![0; s.len() + 1];

    for i in 0..s.len() {
        hashed[i + 1] = ((hashed[i] as u128 * base as u128 + s[i]) % MOD as u128) as u64;
    }

    hashed
}

impl<B: Base, T> RollingHash<B, T> {
    pub fn base(&self) -> u64 {
        self.pow_of_base[1]
    }

    pub fn raw(&self) -> &[T] {
        &self.raw
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    fn hash_sub(&self, from: usize, to: usize) -> u64 {
        ((self.hashed[to] + MOD)
            - ((self.hashed[from] as u128 * self.pow_of_base[to - from] as u128) % MOD as u128)
                as u64)
            % MOD
    }

    pub fn hash<R: RangeBounds<usize>>(&self, range: R) -> u64 {
        let (from, to) = expand_range(range, self.hashed.len());
        self.hash_sub(from, to)
    }

    pub fn longest_common_prefix<R1, R2>(&self, range1: R1, range2: R2) -> &[T]
    where
        R1: RangeBounds<usize>,
        R2: RangeBounds<usize>,
    {
        let (from1, to1) = expand_range(range1, self.len());
        let (from2, to2) = expand_range(range2, self.len());
        let mut l = 0;
        let mut r = min(to1 - from1, to2 - from2) + 1;
        while l + 1 < r {
            let m = (l + r) / 2;
            if self.hash(from1..from1 + m) == self.hash(from2..from2 + m) {
                l = m;
            } else {
                r = m;
            }
        }
        &self.raw[from1..from1 + l]
    }

    pub fn same<R1, R2>(&self, range1: R1, range2: R2) -> bool
    where
        R1: RangeBounds<usize>,
        R2: RangeBounds<usize>,
    {
        let (from1, to1) = expand_range(range1, self.len());
        let (from2, to2) = expand_range(range2, self.len());

        (to1 - from1) == (to2 - from2) && self.hash_sub(from1, to1) == self.hash_sub(from2, to2)
    }
}

impl<B: Base, T: Into<u128> + Clone> RollingHash<B, T> {
    pub fn from_slice(s: &[T]) -> Self {
        let base = B::base();
        let pow_of_base = powering(base, s.len());

        let t = s
            .into_iter()
            .map(|x| x.clone().into())
            .collect::<Vec<u128>>();
        let hashed = rolling(&t, base);

        Self {
            phantom: PhantomData,
            raw: s.to_owned(),
            pow_of_base,
            hashed,
        }
    }
}

impl<B: Base> RollingHash<B, char> {
    pub fn from_str<S: AsRef<str>>(s: S) -> Self {
        let s = s.as_ref().chars().collect::<Vec<_>>();
        let base = B::base();
        let pow_of_base = powering(base, s.len());

        let t = s.iter().map(|x| x.clone().into()).collect::<Vec<u128>>();
        let hashed = rolling(&t, base);

        Self {
            phantom: PhantomData,
            raw: s,
            pow_of_base,
            hashed,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Base, RollingHash};
    use rand::{thread_rng, Rng};

    /// to debug
    #[derive(Debug)]
    struct Base2;
    impl Base for Base2 {
        fn base() -> u64 {
            2
        }
    }

    #[test]
    fn construct() {
        let s = &[1u8, 2, 3, 4, 5, 6, 7];
        let _ = dbg!(RollingHash::<Base2, _>::from_slice(s)); // from slice
    }

    #[test]
    fn random() {
        let mut rng = thread_rng();
        let s = (0..100)
            .map(|_| rng.gen_bool(0.5).to_string().chars().next().unwrap())
            .collect::<String>();
        let n = s.len();
        let rh = RollingHash::<Base2, _>::from_str(s.clone()); // from chars

        for _ in 0..10000 {
            let mut a = rng.gen_range(0..=n);
            let mut b = rng.gen_range(0..=n);
            if b < a {
                std::mem::swap(&mut a, &mut b);
            }

            let shift = rng.gen_range(0..=rh.len() - b);
            let c = a + shift;
            let d = b + shift;

            assert_eq!(rh.same(a..b, c..d), dbg!(&s[a..b]) == dbg!(&s[c..d]));
        }
    }

    #[test]
    fn lcp() {
        let s = "010010101010";
        let rh = RollingHash::<Base2, _>::from_str(s);
        assert_eq!(rh.longest_common_prefix(0.., 1..), []);
        assert_eq!(rh.longest_common_prefix(0.., 2..), ['0']);
        assert_eq!(rh.longest_common_prefix(0.., 3..), ['0', '1', '0']);
        assert_eq!(rh.longest_common_prefix(0.., 4..), []);
        assert_eq!(
            rh.longest_common_prefix(4.., 6..),
            ['1', '0', '1', '0', '1', '0']
        );
        assert_eq!(rh.longest_common_prefix(4..6, 6..), ['1', '0']);
    }
}
