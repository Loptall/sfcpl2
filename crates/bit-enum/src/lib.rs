type BitSet = Vec<bool>;

/// generates bitset
///
/// ```
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
pub struct BitSetIter {
    size: usize,
}

impl BitSetIter {
    pub fn new(n: usize) -> Self {
        Self { size: n }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn combinations(&self) -> usize {
        1 << self.size
    }
}

impl IntoIterator for BitSetIter {
    type Item = BitSet;
    type IntoIter = IntoIterBitSet;
    fn into_iter(self) -> Self::IntoIter {
        IntoIterBitSet {
            size: self.len(),
            current: 0,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct IntoIterBitSet {
    size: usize,
    current: usize,
}

impl Iterator for IntoIterBitSet {
    type Item = BitSet;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == (1 << self.size) {
            None
        } else {
            let res = Some(
                (0..self.size)
                    .rev()
                    .map(|x| self.current & (1 << x) != 0)
                    .collect::<Vec<_>>(),
            );
            self.current += 1;
            res
        }
    }
}

#[cfg(test)]
mod test {
    use super::BitSetIter;

    #[test]
    fn bs_iter() {
        let mut bs = BitSetIter::new(3).into_iter();

        assert_eq!(Some(vec![false, false, false]), bs.next());
        assert_eq!(Some(vec![false, false, true]), bs.next());
        assert_eq!(Some(vec![false, true, false]), bs.next());
        assert_eq!(Some(vec![false, true, true]), bs.next());
        assert_eq!(Some(vec![true, false, false]), bs.next());
        assert_eq!(Some(vec![true, false, true]), bs.next());
        assert_eq!(Some(vec![true, true, false]), bs.next());
        assert_eq!(Some(vec![true, true, true]), bs.next());
        assert_eq!(None, bs.next());
    }
}
