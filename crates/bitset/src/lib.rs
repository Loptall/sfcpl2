use std::{
    borrow::Borrow,
    fmt::{Binary, Debug, Display, Write},
    iter::FromIterator,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index, Not},
};

type Cell = u8;
const CELL_SIZE: usize = std::mem::size_of::<Cell>() * 8;
const ONES: u8 = std::u8::MAX;
const TRUE: &'static bool = &true;
const FALSE: &'static bool = &false;

/// bitset
///
/// - allow length which is not multiple of 8 (size u8)
/// guarantee bits after len are always false
///
/// - allow trailing false
///
///
#[derive(Clone, Default)]
pub struct BitSet {
    inner: Vec<Cell>,
    len: usize,
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

impl Debug for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(self, f)
    }
}

impl From<usize> for BitSet {
    fn from(x: usize) -> Self {
        let inner = (0..std::mem::size_of::<usize>())
            .map(|i| {
                (x << i * CELL_SIZE
                    >> i * CELL_SIZE
                    >> CELL_SIZE * (std::mem::size_of::<usize>() - 1 - i)) as u8
            })
            .collect();
        Self {
            inner,
            len: std::mem::size_of::<usize>() * CELL_SIZE,
        }
    }
}

impl From<u64> for BitSet {
    fn from(x: u64) -> Self {
        let inner = (0..std::mem::size_of::<u64>())
            .map(|i| {
                (x << i * CELL_SIZE
                    >> i * CELL_SIZE
                    >> CELL_SIZE * (std::mem::size_of::<u64>() - 1 - i)) as u8
            })
            .collect();
        Self {
            inner,
            len: std::mem::size_of::<u64>() * CELL_SIZE,
        }
    }
}

impl From<u32> for BitSet {
    fn from(x: u32) -> Self {
        let inner = (0..std::mem::size_of::<u32>())
            .map(|i| {
                (x << i * CELL_SIZE
                    >> i * CELL_SIZE
                    >> CELL_SIZE * (std::mem::size_of::<u32>() - 1 - i)) as u8
            })
            .collect();
        Self {
            inner,
            len: std::mem::size_of::<u32>() * CELL_SIZE,
        }
    }
}

impl From<u16> for BitSet {
    fn from(x: u16) -> Self {
        let inner = (0..std::mem::size_of::<u16>())
            .map(|i| {
                (x << i * CELL_SIZE
                    >> i * CELL_SIZE
                    >> CELL_SIZE * (std::mem::size_of::<u16>() - 1 - i)) as u8
            })
            .collect();
        Self {
            inner,
            len: std::mem::size_of::<u16>() * CELL_SIZE,
        }
    }
}

impl From<u8> for BitSet {
    fn from(x: u8) -> Self {
        let inner = (0..std::mem::size_of::<u8>())
            .map(|i| {
                (x << i * CELL_SIZE
                    >> i * CELL_SIZE
                    >> CELL_SIZE * (std::mem::size_of::<u8>() - 1 - i)) as u8
            })
            .collect();
        Self {
            inner,
            len: std::mem::size_of::<u8>() * CELL_SIZE,
        }
    }
}

impl From<&[u8]> for BitSet {
    fn from(v: &[u8]) -> Self {
        Self {
            len: v.len() * CELL_SIZE,
            inner: v.to_vec(),
        }
    }
}

impl From<Vec<u8>> for BitSet {
    fn from(v: Vec<u8>) -> Self {
        Self {
            len: v.len() * CELL_SIZE,
            inner: v,
        }
    }
}

impl From<&Vec<u8>> for BitSet {
    fn from(v: &Vec<u8>) -> Self {
        Self {
            len: v.len() * CELL_SIZE,
            inner: v.clone(),
        }
    }
}

impl From<Vec<bool>> for BitSet {
    fn from(v: Vec<bool>) -> Self {
        let mut res = BitSet::zeros(v.len());
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

// impl<B> Index<B> for BitSet
// where
//     B: RangeBounds<usize>,
// {
//     type Output;

//     fn index(&self, index: RangeBounds<usize>) -> &Self::Output {
//         todo!()
//     }
// }

impl PartialEq for BitSet {
    fn eq(&self, other: &Self) -> bool {
        self.is_subset(other) && other.is_subset(self)
    }
}

impl Eq for BitSet {}

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
        self.swap();
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
    /// create new BitSet filled with false
    ///
    /// alias to `BitSet::zeros(len)`
    pub fn new(len: usize) -> Self {
        Self::zeros(len)
    }

    pub fn clear(&mut self) {
        if let Some((last, elems)) = self.inner.split_last_mut() {
            for el in elems {
                *el = 0;
            }

            *last = 0;
        }
    }

    pub fn fill(&mut self) {
        if let Some((last, elems)) = self.inner.split_last_mut() {
            for el in elems {
                *el = ONES;
            }

            *last = ONES;
        }

        self.chomp();
    }

    /// create new BitSet filled with false
    pub fn zeros(len: usize) -> Self {
        Self {
            inner: vec![0; (len - 1 >> 3) + 1],
            len,
        }
    }

    /// create new BitSet filled with true
    pub fn ones(len: usize) -> Self {
        let mut res = Self {
            inner: vec![ONES; (len - 1 >> 3) + 1],
            len,
        };
        res.chomp();
        res
    }

    pub fn from_iter(iter: impl Iterator<Item = usize>) -> Self {
        let buf: Vec<_> = iter.into_iter().collect();
        let len = buf.iter().max().unwrap_or(&0) + 1;
        let mut bs = BitSet::new(len);
        for x in buf {
            bs.entry(x);
        }
        bs
    }

    pub fn from_uint_with_len<T: Into<usize>>(x: T, len: usize) -> Self {
        let x: usize = x.into();
        if x >> len != 0 {
            panic!("given integer has standing bit out of lower given length")
        }
        let mut res = Self::zeros(len);
        for i in 0..len {
            if x & (1usize << i) != 0 {
                res.entry(res.len - i - 1);
            }
        }
        res
    }

    pub fn resize(&mut self, len: usize) {
        self.inner.resize(len + 7 >> 3, 0);
        self.len = len;
    }

    /// return length of inner cells
    pub fn cell(&self) -> usize {
        self.inner.len()
    }

    /// return length of bits which can be use
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn count_ones(&self) -> usize {
        self.inner
            .iter()
            .map(|&x| x.count_ones() as usize)
            .sum::<usize>()
    }

    pub fn count_zeros(&self) -> usize {
        self.inner
            .iter()
            .map(|&x| x.count_zeros() as usize)
            .sum::<usize>()
            - {
                // zeros after len are ignored
                let r = self.len() % CELL_SIZE;
                if r != 0 {
                    CELL_SIZE - r
                } else {
                    0
                }
            }
    }

    pub fn is_empty(&self) -> bool {
        self.count_ones() == 0
    }

    /// convert index in the way of cells
    ///
    /// returns (cell, position)
    ///
    /// where idx == cell * 8 + position is always valid
    fn assert_index(&self, idx: usize) -> Option<(usize, usize)> {
        if self.len() <= idx {
            None
        } else {
            Some((idx / 8, idx % 8))
        }
    }

    pub fn get(&self, idx: usize) -> bool {
        if let Some((cell, position)) = self.assert_index(idx) {
            self.inner[cell] & (1 << 7 >> position) != 0
        } else {
            false // is it good idea to return false over index?
        }
    }

    pub fn entry(&mut self, idx: usize) {
        let (cell, position) = self.assert_index(idx).expect(
            format!("invalid index: len is {}, but index is {}", self.len(), idx,).as_str(),
        );
        self.inner[cell] |= 1 << 7 >> position;
    }

    pub fn remove(&mut self, idx: usize) {
        let (cell, position) = self.assert_index(idx).expect(
            format!("invalid index: len is {}, but index is {}", self.len(), idx,).as_str(),
        );
        self.inner[cell] &= ONES ^ (1 << 7 >> position);
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

    pub fn swap(&mut self) {
        for x in self.inner.iter_mut() {
            *x = x.swap_bytes();
        }
        self.chomp();
    }

    fn chomp(&mut self) {
        // len == 5
        // bytes = "10110/110" -> "10110/000"
        // self & "11111/000"
        let r = CELL_SIZE - self.len() % CELL_SIZE;
        let r = if r == CELL_SIZE { 0 } else { r };
        if let Some(last) = self.inner.last_mut() {
            *last &= ONES >> r << r;
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::BitSet;
    use rand::{thread_rng, Rng};

    #[test]
    fn basic() {
        let mut bs = BitSet::new(10);

        bs.entry(1);
        bs.entry(3);

        assert_eq!(bs.inner, vec![(1 << 6) + (1 << 4), 0]);

        bs.remove(3);
        bs.remove(1);

        assert_eq!(bs.inner, vec![0, 0]);
    }

    #[test]
    fn large() {
        let limit = 200000;
        let mut bs = BitSet::new(limit);
        let mut rng = thread_rng();

        let mut pops = BTreeSet::new();
        for _ in (0..).take(10000) {
            let r: usize = rng.gen_range(0..limit);
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
            let mut t = BitSet::new(4);
            t.entry(0);
            t.entry(3);
            t
        });
    }

    #[test]
    fn iter() {
        let mut bs = BitSet::new(10);
        bs.entry(1);
        bs.entry(2);
        bs.entry(4);
        bs.entry(5);

        let iter1 = bs.iter();
        let mut iter2 = iter1.clone();
        assert_eq!(iter2.next(), Some(1));
        assert_eq!(iter2.next(), Some(2));
        assert_eq!(iter2.next(), Some(4));
        assert_eq!(iter2.next(), Some(5));
        assert_eq!(iter2.next(), None);

        let bs_from_iter: BitSet = iter1.collect();
        // eprintln!("{}", bs);
        // eprintln!("{}", bs_from_iter);
        assert_eq!(bs, bs_from_iter);
    }

    #[test]
    fn from_uint() {
        let mut bs = BitSet::new(64);
        bs.entry(8);
        bs.entry(63);
        let from_usize = ((1usize << 55) + (1usize)).into();
        assert_eq!(bs, from_usize);
    }
}
