use std::{
    borrow::Borrow,
    fmt::{Binary, Display, Write},
    iter::{repeat, FromIterator},
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not},
};

type Frame = u64;

const ONES: u64 = std::u64::MAX;
const TRUE: &'static bool = &true;
const FALSE: &'static bool = &false;

/// flexible heap-allocated bitset
///
///
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BitSet {
    inner: Vec<Frame>,
    // len: usize,
}

impl Binary for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            f.write_str("0b")?;
        }

        for i in 0..self.len() {
            if self[i] {
                f.write_char('1')?;
            } else {
                f.write_char('0')?;
            }
        }

        Ok(())
    }
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(self, f)
    }
}

impl From<Vec<bool>> for BitSet {
    fn from(v: Vec<bool>) -> Self {
        let mut res = BitSet::with_len(v.len());
        for (x, _) in v.iter().enumerate().filter(|x| *x.1) {
            res.entry(x);
        }
        res
    }
}

impl From<Vec<usize>> for BitSet {
    fn from(v: Vec<usize>) -> Self {
        v.into_iter().collect()
    }
}

impl Index<usize> for BitSet {
    type Output = bool;
    fn index(&self, idx: usize) -> &Self::Output {
        if self.get(idx) {
            TRUE
        } else {
            FALSE
        }
    }
}

impl Index<&usize> for BitSet {
    type Output = bool;
    fn index(&self, idx: &usize) -> &Self::Output {
        if self.get(*idx) {
            TRUE
        } else {
            FALSE
        }
    }
}

impl BitAndAssign<BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: BitSet) {
        for (a, b) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *a &= b;
        }
    }
}

impl BitAndAssign<&BitSet> for BitSet {
    fn bitand_assign(&mut self, rhs: &BitSet) {
        for (a, b) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *a &= b;
        }
    }
}

impl BitAnd<BitSet> for BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: BitSet) -> Self::Output {
        let mut res = self.clone();
        res &= rhs;
        res
    }
}

impl BitAnd<BitSet> for &BitSet {
    type Output = BitSet;
    fn bitand(self, rhs: BitSet) -> Self::Output {
        let mut res = self.clone();
        res &= rhs;
        res
    }
}

impl BitOrAssign<BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: BitSet) {
        for (a, b) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *a |= b;
        }
    }
}

impl BitOrAssign<&BitSet> for BitSet {
    fn bitor_assign(&mut self, rhs: &BitSet) {
        for (a, b) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *a |= b;
        }
    }
}

impl BitOr<BitSet> for BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: BitSet) -> Self::Output {
        let mut res = self.clone();
        res |= rhs;
        res
    }
}

impl BitOr<BitSet> for &BitSet {
    type Output = BitSet;
    fn bitor(self, rhs: BitSet) -> Self::Output {
        let mut res = self.clone();
        res |= rhs;
        res
    }
}

impl BitXorAssign<BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: BitSet) {
        for (a, b) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *a ^= b;
        }
    }
}

impl BitXorAssign<&BitSet> for BitSet {
    fn bitxor_assign(&mut self, rhs: &BitSet) {
        for (a, b) in self.inner.iter_mut().zip(rhs.inner.iter()) {
            *a ^= b;
        }
    }
}

impl BitXor<BitSet> for BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: BitSet) -> Self::Output {
        let mut res = self.clone();
        res ^= rhs;
        res
    }
}

impl BitXor<BitSet> for &BitSet {
    type Output = BitSet;
    fn bitxor(self, rhs: BitSet) -> Self::Output {
        let mut res = self.clone();
        res ^= rhs;
        res
    }
}

impl Not for BitSet {
    type Output = BitSet;
    fn not(mut self) -> Self::Output {
        self.negate();
        self
    }
}

impl Not for &BitSet {
    type Output = BitSet;
    fn not(self) -> Self::Output {
        self.clone().not()
    }
}

impl BitSet {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_len(len: usize) -> Self {
        let mut res = Self {
            inner: Vec::with_capacity((len >> 6) + 1),
        };
        res.resize(len);
        res
    }

    pub fn from_iter(iter: impl Iterator<Item = usize>) -> Self {
        let mut bs = BitSet::new();
        for x in iter {
            bs.entry(x);
        }
        bs
    }

    pub fn frames(&self) -> usize {
        self.inner.len()
    }

    pub fn len(&self) -> usize {
        self.inner.len() * (1 << 6)
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity() << 6
    }

    pub fn count_ones(&self) -> usize {
        self.inner
            .iter()
            .fold(0, |sum, elem| sum + elem.count_ones() as usize)
    }

    // pub fn count_zeroes(&self) -> usize {
    //     self.inner
    //         .iter()
    //         .fold(0, |sum, elem| sum + elem.count_zeros() as usize)
    // }

    pub fn is_empty(&self) -> bool {
        self.count_ones() == 0
    }

    fn reserve_least(&mut self, least_idx: usize) {
        if self.len() == 0 {
            self.inner = vec![0; 1];
        }
        while self.frames() * (1 << 6) <= least_idx {
            self.inner.extend(repeat(0).take(self.frames()));
        }
    }

    pub fn resize(&mut self, len: usize) {
        self.inner.resize((len >> 6) + 1, 0);
    }

    pub fn truncate(&mut self, len: usize) {
        self.inner.truncate(len);
    }

    pub fn get(&self, idx: usize) -> bool {
        let (frame, position) = frame_index(idx);
        self.inner
            .get(frame)
            .map(|&f| f & (1 << position) != 0)
            .expect(&format!(
                "invalid index: len is {}, but index is {}",
                self.len(),
                idx
            ))
    }

    pub fn entry(&mut self, idx: usize) {
        self.reserve_least(idx);
        let (frame, position) = frame_index(idx);
        self.inner[frame] |= 1 << position;
    }

    pub fn remove(&mut self, idx: usize) {
        self.reserve_least(idx);
        let (frame, position) = frame_index(idx);
        self.inner[frame] &= ONES ^ (1 << position);
    }

    pub fn set(&mut self, idx: usize, value: bool) {
        if value {
            self.entry(idx);
        } else {
            self.remove(idx);
        }
    }

    pub fn flip(&mut self, idx: usize) {
        self.set(idx, !self.get(idx));
    }

    pub fn iter(&self) -> BitSetIter<&Self> {
        BitSetIter {
            inner: self,
            cur: 0,
        }
    }

    pub fn negate(&mut self) {
        for x in self.inner.iter_mut() {
            *x = x.swap_bytes();
        }
    }

    pub fn is_subset(&self, other: &Self) -> bool {
        if self.len() <= other.len() {
            self.iter().all(|x| other[x])
        } else {
            other.iter().all(|x| self[x])
        }
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        other.is_subset(self)
    }

    pub fn is_disjoint(&self, other: &Self) -> bool {
        if self.len() <= other.len() {
            self.iter().all(|x| !other[x])
        } else {
            other.iter().all(|x| !self[x])
        }
    }

    pub fn union(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res |= other;
        res
    }

    pub fn intersection(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res &= other;
        res
    }

    pub fn difference(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res &= !other;
        res
    }

    pub fn symmetric_difference(&self, other: &Self) -> Self {
        let mut res = self.clone();
        res ^= other;
        res
    }
}

#[derive(Debug, Clone)]
pub struct BitSetIter<B> {
    inner: B,
    cur: usize,
}

impl<B: Borrow<BitSet>> Iterator for BitSetIter<B> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.cur < self.inner.borrow().len() && {
            if self.inner.borrow()[self.cur] {
                self.cur += 1;
                return Some(self.cur - 1);
            } else {
                self.cur += 1;
                true
            }
        } {}

        None
    }
}

impl FromIterator<usize> for BitSet {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        <Self>::from_iter(iter.into_iter())
    }
}

impl IntoIterator for BitSet {
    type Item = usize;
    type IntoIter = BitSetIter<BitSet>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            inner: self,
            cur: 0,
        }
    }
}

/// convert index in the way of frames
///
/// returns (frame, position)
///
/// where idx == frame * 64 + position is always valid
fn frame_index(idx: usize) -> (usize, usize) {
    (idx >> 6, (1 << 6) - (idx + (1 << 6) - 1) % (1 << 6) - 1)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::BitSet;
    use rand::{thread_rng, Rng};

    #[test]
    fn basic() {
        let mut bs = BitSet::new();

        bs.entry(1);
        bs.entry(3);

        assert_eq!(bs.len(), 64);
        assert_eq!(bs.inner, vec![(1 << 63) + (1 << 61)]);

        bs.remove(3);
        bs.remove(1);

        assert_eq!(bs.inner, vec![0]);
    }

    #[test]
    fn large() {
        let mut bs = BitSet::new();
        let mut rng = thread_rng();

        let mut pops = BTreeSet::new();
        for _ in (0..).take(10000) {
            let r: usize = rng.gen_range(0..200000);
            pops.insert(r);
            bs.entry(r);
        }

        assert_eq!(pops.len(), bs.count_ones());
        assert!(pops.into_iter().all(|x| bs[x]));
    }

    #[test]
    fn from_vec() {
        let vec = vec![true, false, false, true];
        let bs: BitSet = vec.into();
        assert_eq!(bs, {
            let mut t = BitSet::new();
            t.entry(0);
            t.entry(3);
            t
        });
    }

    #[test]
    fn iter() {
        let mut bs = BitSet::new();
        bs.entry(1);
        bs.entry(2);
        bs.entry(4);
        bs.entry(5);

        let mut iter = bs.iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }
}
