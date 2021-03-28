use std::marker::PhantomData;

pub trait Sort {
    fn sort<T: Ord>(v: &mut [T]);
}

struct DefaultSort;
impl Sort for DefaultSort {
    fn sort<T: Ord>(v: &mut [T]) {
        v.sort();
    }
}

struct SaIs;
impl Sort for SaIs {
    fn sort<T: Ord>(_v: &mut [T]) {
        todo!()
    }
}

/// String index
pub struct SuffixArray<'a, S> {
    phantom: PhantomData<S>,
    raw: &'a str,
    inner: Vec<&'a str>,
}

impl<'a, S: Sort> SuffixArray<'a, S> {
    /// build suffix array from &str
    ///
    /// sort all suffixes of `s`
    ///
    /// Complexity: O(SK)
    ///
    /// where
    /// - S = complexity of sort algorythm
    /// - K = length(s)
    pub fn new(s: &'a str) -> Self {
        let mut inner = Vec::new();
        for i in 0..s.len() {
            inner.push(&s[i..]);
        }

        S::sort(&mut inner);

        Self {
            phantom: PhantomData,
            raw: s,
            inner,
        }
    }

    pub fn raw(&self) -> &str {
        self.raw
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    fn suffix_nth(&self, n: usize) -> &str {
        self.inner[n]
    }

    /// Returns all start index where t appears in s.
    ///
    /// Be careful that returned vector is not sorted.
    ///
    /// Complexity: O(log K)
    ///
    /// where
    /// - K = length(s)
    pub fn find<A: AsRef<str>>(&self, t: A) -> Vec<usize> {
        let t = t.as_ref();
        let n = self.len();
        let mut l = 0;
        let mut r = n;
        while l + 1 < r {
            let m = (l + r) / 2;
            if self.suffix_nth(m) < t {
                l = m;
            } else {
                r = m;
            }
        }

        (r..n)
            .map(|x| self.suffix_nth(x))
            .take_while(|&s| s.starts_with(t))
            .map(|s| n - s.len())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::{DefaultSort, SuffixArray};
    #[test]
    fn basic() {
        let s = "abracadabra";
        let suffix_array = SuffixArray::<DefaultSort>::new(s);
        assert_eq!(suffix_array.find("br"), vec![8, 1]);
        assert_eq!(suffix_array.find("abra"), vec![7, 0]);
    }
}
