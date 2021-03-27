/// generates `Vec<bool>`
///
/// ```ignore
/// for bs in BruteBitsBuilder::new(3) {
///    // bs iterate from [false, false, false] to [true, true, true]
/// }
/// ```
#[derive(Debug, Copy, Clone)]
pub struct BruteBitsBuilder {
    size: usize,
}

impl BruteBitsBuilder {
    pub fn new(n: usize) -> Self {
        Self { size: n }
    }
}

impl IntoIterator for BruteBitsBuilder {
    type Item = Vec<bool>;
    type IntoIter = BruteBits;
    fn into_iter(self) -> Self::IntoIter {
        BruteBits {
            size: self.size,
            current: 0,
        }
    }
}

/// light iterator
#[derive(Copy, Clone, Debug)]
pub struct BruteBits {
    size: usize,
    current: usize,
}

impl Iterator for BruteBits {
    type Item = Vec<bool>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current == (1 << self.size) {
            None
        } else {
            let mut res = vec![false; self.size];
            for i in 0..self.size {
                if (1 << i) & self.current != 0 {
                    res[i] = true;
                }
            }
            self.current += 1;
            Some(res)
        }
    }
}

#[cfg(test)]
mod test {
    use super::BruteBitsBuilder;

    #[test]
    fn bs_iter() {
        let mut bs = BruteBitsBuilder::new(3).into_iter();

        assert_eq!(Some(vec![false, false, false]), dbg!(bs.next()));
        assert_eq!(Some(vec![true, false, false]), dbg!(bs.next()));
        assert_eq!(Some(vec![false, true, false]), dbg!(bs.next()));
        assert_eq!(Some(vec![true, true, false]), dbg!(bs.next()));
        assert_eq!(Some(vec![false, false, true]), dbg!(bs.next()));
        assert_eq!(Some(vec![true, false, true]), dbg!(bs.next()));
        assert_eq!(Some(vec![false, true, true]), dbg!(bs.next()));
        assert_eq!(Some(vec![true, true, true]), dbg!(bs.next()));
        assert_eq!(None, bs.next());
    }
}
