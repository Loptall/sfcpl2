use digitex::Binary;
use std::{fmt::Display, iter::repeat};

type Frame = u64;

const ONES: u64 = std::u64::MAX;

/// flexible heap-allocated bitset
///
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitSet {
    inner: Vec<Frame>,
}

impl Display for BitSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for &frame in self.inner.iter() {
            if frame == 0 {
                f.write_str(&"0".repeat(1 << 6))?
            } else {
                write!(f, "{}", Binary::from_raw(frame))?
            }
        }
        Ok(())
    }
}

impl From<Vec<bool>> for BitSet {
    fn from(v: Vec<bool>) -> Self {
        let mut res = BitSet::with_capacity(v.len());
        for (i, frame) in v.chunks(1 << 6).enumerate() {
            for position in 0..(1 << 6) {
                let idx = i * (1 << 6) + position;
                if let Some(&val) = frame.get(position) {
                    if val {
                        res.entry(idx);
                    }
                }
            }
        }

        res
    }
}

// impl Index<usize> for BitSet {
//     type Output = bool;
//     fn index(&self, idx: usize) -> &Self::Output {
//         unimplemented!()
//     }
// }

impl BitSet {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            inner: Vec::with_capacity((size >> 6) + 1),
        }
    }

    pub fn frames(&self) -> usize {
        self.inner.len()
    }

    pub fn len(&self) -> usize {
        self.frames() << 6
    }

    pub fn capacity(&self) -> usize {
        self.inner.capacity() << 6
    }

    pub fn element_count(&self) -> usize {
        self.inner
            .iter()
            .fold(0, |sum, elem| sum + elem.count_ones() as usize)
    }

    pub fn is_empty(&self) -> bool {
        self.element_count() == 0
    }

    fn double_len(&mut self) {
        self.inner.extend(repeat(0).take(self.frames()));
    }

    fn reserve_least(&mut self, least_idx: usize) {
        if self.len() == 0 {
            self.inner = vec![0; 1];
        }
        while self.len() <= least_idx {
            self.double_len();
        }
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
        let (frame, position) = frame_index(idx);
        if let Some(f) = self.inner.get_mut(frame) {
            *f &= ONES - (1 << position);
        }
    }
}

/// convert index in the way of frames
///
/// returns (frame, position)
///
/// where idx == frame * 64 + position is always valid
fn frame_index(idx: usize) -> (usize, usize) {
    // (idx >> 6, (1 << 6) - (idx - (idx >> 6 << 6)))
    // dbg!((1 << 6) - 1 - (idx + (1 << 6) - 1) % (1 << 6));
    (idx >> 6, (1 << 6) - (idx + (1 << 6) - 1) % (1 << 6) - 1)
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::BitSet;
    use rand::{thread_rng, Rng};

    #[test]
    fn get() {
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

        assert_eq!(pops.len(), bs.element_count());
        assert!(pops.into_iter().all(|x| bs.get(x)));
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
}
