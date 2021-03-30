use std::{collections::BTreeMap, marker::PhantomData};

use util::{
    traits::{BoundedBelow, IntoVec},
    Shuffle as _, UniqueCount as _,
};

pub trait Sort<T> {
    fn sort(s: &[T]) -> Vec<usize>;
}

#[derive(Debug)]
pub struct DefaultSort;
impl<T: Ord> Sort<T> for DefaultSort {
    fn sort(s: &[T]) -> Vec<usize> {
        let mut inner = (0..=s.len()).collect::<Vec<_>>();
        inner.sort_by_key(|&x| &s[x..]);
        inner
    }
}

#[derive(Debug)]
pub struct SaIs;
impl<T: Ord + BoundedBelow + Clone> Sort<T> for SaIs {
    fn sort(s: &[T]) -> Vec<usize> {
        fn induced_sort<U: Ord + BoundedBelow + Clone>(
            s: &[U],
            types: &[bool],
            seed: &[usize],
        ) -> Vec<usize> {
            let is_lms = |x: &usize| *x != 0 && types[*x] == S && types[*x - 1] == L;
            let is_l = |x: &usize| types[*x];
            let is_s = |x: &usize| !types[*x];

            let n = s.len();
            let mut res = vec![None; n];
            let bin = {
                let mut sum = 0;
                s.unique_count()
                    .into_iter()
                    .map(|(c, x)| {
                        let r = sum;
                        sum += x;
                        (c, (r, sum))
                    })
                    .collect::<BTreeMap<_, _>>()
            };

            // find lms
            let mut inserted_lms = BTreeMap::<U, usize>::new();
            for &i in seed {
                res[bin[&s[i]].1 - 1 - {
                    let h = inserted_lms.entry(s[i].clone()).or_default();
                    let r = *h;
                    *h += 1;
                    r
                }] = Some(i);
            }

            // find L
            let mut inserted_l = BTreeMap::<U, usize>::new();
            for i in 0..n {
                if let Some(x) = res[i] {
                    if x > 0 && is_l(&(x - 1)) {
                        res[bin[&s[x - 1]].0 + {
                            let h = inserted_l.entry(s[x - 1].clone()).or_default();
                            let r = *h;
                            *h += 1;
                            r
                        }] = Some(x - 1);
                    }

                    if i > 0 && is_lms(&res[i].unwrap()) {
                        res[i] = None;
                    }
                }
            }

            // rfind S
            let mut inserted_s = BTreeMap::<U, usize>::new();
            for i in (0..n).rev() {
                if let Some(x) = res[i] {
                    if x > 0 && is_s(&(x - 1)) {
                        res[bin[&s[x - 1]].1 - 1 - {
                            let h = inserted_s.entry(s[x - 1].clone()).or_default();
                            let r = *h;
                            *h += 1;
                            r
                        }] = Some(x - 1);
                    }
                }
            }

            res.into_iter().filter_map(|x| x).collect()
        }

        const L: bool = true;
        const S: bool = false;

        let mut s = s.to_vec();
        s.push(T::min_value());
        let s = s;
        let n = s.len();

        // determine s[i] is L or S, L: true, S: false
        let types = {
            let mut r = vec![false; n];
            (0..n - 1).rev().for_each(|i| {
                r[i] = if s[i] == s[i + 1] {
                    r[i + 1]
                } else {
                    s[i] > s[i + 1]
                };
            });
            r
        };

        let is_lms =
            |x: &usize| *x >= types.len() - 1 || (*x > 0 && types[*x] == S && types[*x - 1] == L);

        // temporary seed
        let lmss = (0..n).filter(is_lms).collect::<Vec<_>>();
        let seed = lmss.shuffle();

        // first sort
        let res = induced_sort(&s, &types, &seed);

        let lms = res
            .iter()
            .filter(|&i| is_lms(i))
            .map(|x| *x)
            .collect::<Vec<_>>();
        let m = lms.len();
        let mut idx = vec![None; n];
        idx[lms[0]] = Some(0);
        let mut count = 0;

        for (i, j) in (0..m - 1).map(|x| (lms[x], lms[x + 1])) {
            for d in 0..n {
                if s[i + d] != s[j + d] || is_lms(&(i + d)) != is_lms(&(j + d)) {
                    count += 1;
                    break;
                }
                if d > 0 && (is_lms(&(i + d)) || is_lms(&(j + d))) {
                    break;
                }
            }
            idx[j] = Some(count);
        }

        let idx = idx.into_iter().filter_map(|x| x).collect::<Vec<_>>();

        // actual seed
        let seed: Vec<usize> = if count + 1 >= idx.len() {
            let mut r = vec![0; m];
            for (i, c) in idx.into_iter().enumerate() {
                r[c] = i;
            }
            r.into_iter().map(|x| lms[x]).collect()
        } else {
            let idx = idx.into_iter().take(m - 1).collect::<Vec<usize>>();
            SaIs::sort(&idx).into_iter().map(|x| lms[x]).collect()
        };

        // second sort
        let res = induced_sort(&s, &types, &seed);
        res
    }
}

/// String index
///
/// `self.inner` include all suffix of `self.raw`, include empty string and its own.
///
/// takes 2 type parameters T and S.
/// where.
///
/// - T = element's type of slice, espetialy `char` if str is given.
/// - S = algorythm to sort the suffix, SA-IS is linear implementation
///
/// ```
/// use suffix_array::{SuffixArray, SaIs};
///
/// let s = "abracadabra";
/// assert_eq!(s.len(), 11);
///
/// let sa = SuffixArray::<char, SaIs>::from_str(s);
/// assert_eq!(sa.len(), 12);
///
/// assert_eq!(
///     sa.suffix_array(),
///     vec![
///         11, // ""
///         10, // "a"
///         7,  // "abra"
///         0,  // "abracadabra"
///         3,  // "acadabra"
///         5,  // "adabra"
///         8,  // "bra"
///         1,  // "bracadabra"
///         4,  // "cadabra"
///         6,  // "dabra"
///         9,  // "ra"
///         2,  // "racadabra"
///     ]);
/// ```
#[derive(Debug)]
pub struct SuffixArray<T, S> {
    phantom: PhantomData<S>,
    raw: Vec<T>,
    inner: Vec<usize>,
}

impl<T: Ord + Clone, S: Sort<T>> SuffixArray<T, S> {
    /// Build suffix array from &str
    ///
    /// If you want to build from str type,
    /// you can use 1SuffixArray::from_str()1
    ///
    /// Complexity: O(NS)
    ///
    /// where
    /// - S = complexity of sort algorythm
    /// - N = length(s)
    pub fn new(s: &[T]) -> Self {
        let inner = S::sort(s);
        Self {
            phantom: PhantomData,
            raw: s.to_vec(),
            inner,
        }
    }

    /// Returns raw slice given first
    pub fn raw(&self) -> &[T] {
        &self.raw
    }

    /// Length of `self.inner`
    ///
    /// This is equal to `self.raw.len() + 1`, because `self.inner` includes all suffix.
    /// Be careful that this is not same to `raw.len()`
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns nth suffix.
    ///
    /// 0th = always "" (empty string)
    ///
    /// # Panic
    ///
    /// If n >= self.len()
    pub fn suffix_nth(&self, n: usize) -> &[T] {
        &self.raw[self.inner[n]..]
    }

    /// Represents suffix array in its suffix's start indices
    pub fn suffix_array(&self) -> &[usize] {
        &self.inner
    }

    /// Returns all start index where t appears in s.
    ///
    /// Be careful that returned vector is not sorted.
    ///
    /// Complexity: O(M log N)
    ///
    /// where
    /// - N = length(s)
    /// - M = length(t)
    pub fn find<V: IntoVec<T>>(&self, t: V) -> &[usize] {
        let t = t.into();
        let n = self.len();
        let mut l1 = 0;
        let mut r1 = n;
        while l1 + 1 < r1 {
            let m = (l1 + r1) / 2;
            if self.suffix_nth(m) < &t {
                l1 = m;
            } else {
                r1 = m;
            }
        }

        let mut l2 = r1;
        let mut r2 = n;
        while l2 + 1 < r2 {
            let m = (l2 + r2) / 2;
            if self.suffix_nth(m).starts_with(&t) {
                l2 = m;
            } else {
                r2 = m;
            }
        }

        &self.inner[r1..r2]
    }
}

impl<S: Sort<char>> SuffixArray<char, S> {
    pub fn from_str(s: &str) -> Self {
        let t = s.chars().collect::<Vec<char>>();
        let inner = S::sort(&t);
        Self {
            phantom: PhantomData,
            raw: t,
            inner,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{DefaultSort, SaIs, SuffixArray};

    #[test]
    fn basic() {
        let s = "abracadabra".to_string();
        let suffix_array = SuffixArray::<char, DefaultSort>::from_str(&s);
        assert_eq!(suffix_array.find("br"), vec![8, 1]);
        assert_eq!(suffix_array.find("abra"), vec![7, 0]);
        assert_eq!(suffix_array.find("r"), vec![9, 2]);

        let s = "mmiissiissiippii".to_string();
        let suffix_array = SuffixArray::<char, DefaultSort>::from_str(&s);
        for i in 0..s.len() - 1 {
            assert!(suffix_array.suffix_nth(i) < suffix_array.suffix_nth(i + 1));
        }
    }

    #[test]
    fn sa_is() {
        let s = "abracadabra".to_string();
        let suffix_array = SuffixArray::<char, SaIs>::from_str(&s);
        assert_eq!(suffix_array.find("br"), vec![8, 1]);
        assert_eq!(suffix_array.find("abra"), vec![7, 0]);
        assert_eq!(suffix_array.find("r"), vec![9, 2]);

        let s = "mmiissiissiippii".to_string();
        let suffix_array = SuffixArray::<char, SaIs>::from_str(&s);
        for i in 0..s.len() - 1 {
            assert!(suffix_array.suffix_nth(i) < suffix_array.suffix_nth(i + 1));
        }
    }
}
