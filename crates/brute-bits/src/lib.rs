use bitset::BitSet;

/// generates bitset
///
/// ```ignore
/// use bit_enum::BitSetIter;
///
/// for bs in BitSetIter::new(3) {
///    // bs iterate from [false, false, false] to [true, true, true]
/// }
/// ```
///
/// # Panic
/// If `size > 64`, usize will overflow and panic
#[derive(Debug, Copy, Clone)]
pub struct BruteBitsBuilder {
    size: usize,
}

impl BruteBitsBuilder {
    pub fn new(n: usize) -> Self {
        Self { size: n }
    }

    // pub fn len(&self) -> usize {
    //     self.size
    // }

    // pub fn combinations(&self) -> usize {
    //     1 << self.size
    // }
}

impl IntoIterator for BruteBitsBuilder {
    type Item = BitSet;
    type IntoIter = BruteBits;
    fn into_iter(self) -> Self::IntoIter {
        BruteBits {
            size: self.size,
            current: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct BruteBits {
    size: usize,
    current: usize,
}

impl Iterator for BruteBits {
    type Item = BitSet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == (1 << self.size) {
            None
        } else {
            let res = Some(BitSet::from_uint_with_len(self.current, self.size));
            self.current += 1;
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::BruteBitsBuilder;

    #[test]
    fn bs_iter() {
        let mut bs = BruteBitsBuilder::new(3).into_iter();

        assert_eq!(Some(vec![false, false, false].into()), bs.next());
        assert_eq!(Some(vec![false, false, true].into()), bs.next());
        assert_eq!(Some(vec![false, true, false].into()), bs.next());
        assert_eq!(Some(vec![false, true, true].into()), bs.next());
        assert_eq!(Some(vec![true, false, false].into()), bs.next());
        assert_eq!(Some(vec![true, false, true].into()), bs.next());
        assert_eq!(Some(vec![true, true, false].into()), bs.next());
        assert_eq!(Some(vec![true, true, true].into()), bs.next());
        assert_eq!(None, bs.next());
    }
}
