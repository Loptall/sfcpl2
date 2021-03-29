use std::{collections::BTreeMap, marker::PhantomData};

use unique_count::UniqueCount as _;
use util::{BoundedBelow, IntoVec, Shuffle as _};

pub trait Sort<T> {
    fn sort(s: &[T]) -> Vec<usize>;
}

#[derive(Debug)]
struct DefaultSort;
impl<T: Ord> Sort<T> for DefaultSort {
    fn sort(s: &[T]) -> Vec<usize> {
        let mut inner = Vec::new();
        for i in 0..s.len() {
            inner.push(&s[i..]);
        }

        inner.sort();
        inner.into_iter().map(|t| s.len() - t.len()).collect()
    }
}

#[derive(Debug)]
struct SaIs;
impl<T: Ord + BoundedBelow + Clone + std::fmt::Debug> Sort<T> for SaIs {
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
            r.into_iter().map(|x| res[x]).collect()
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
#[derive(Debug)]
pub struct SuffixArray<T, S> {
    phantom: PhantomData<S>,
    raw: Vec<T>,
    inner: Vec<usize>,
}

impl<T: Ord + Clone, S: Sort<T>> SuffixArray<T, S> {
    /// build suffix array from &str
    ///
    /// sort all suffixes of `s`
    ///
    /// Complexity: O(SK)
    ///
    /// where
    /// - S = complexity of sort algorythm
    /// - K = length(s)
    pub fn new(s: &[T]) -> Self {
        Self {
            phantom: PhantomData,
            raw: s.to_vec(),
            inner: S::sort(s),
        }
    }

    pub fn raw(&self) -> &[T] {
        &self.raw
    }

    pub fn len(&self) -> usize {
        self.raw.len()
    }

    fn suffix_nth(&self, n: usize) -> &[T] {
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
    /// Complexity: O(log K)
    ///
    /// where
    /// - K = length(s)
    pub fn find<V: IntoVec<T>>(&self, t: V) -> Vec<usize> {
        let t = t.into();
        let n = self.len();
        let mut l = 0;
        let mut r = n;
        while l + 1 < r {
            let m = (l + r) / 2;
            if self.suffix_nth(m) < &t {
                l = m;
            } else {
                r = m;
            }
        }

        (r..n)
            .map(|x| self.suffix_nth(x))
            .take_while(|&s| s.starts_with(&t))
            .map(|s| n - s.len())
            .collect()
    }
}

impl<S: Sort<char>> SuffixArray<char, S> {
    pub fn from_str(s: &String) -> Self {
        let t = s.chars().collect::<Vec<char>>();
        Self {
            phantom: PhantomData,
            inner: S::sort(&t),
            raw: t,
        }
    }
}

#[cfg(test)]
mod test {
    // use proconio::input;

    use super::{DefaultSort, SaIs, SuffixArray};
    #[test]
    fn basic() {
        let s = "abracadabra".to_string();
        let suffix_array = SuffixArray::<char, DefaultSort>::from_str(&s);
        assert_eq!(suffix_array.find("br"), vec![8, 1]);
        assert_eq!(suffix_array.find("abra"), vec![7, 0]);
    }

    #[test]
    fn sa_is() {
        let s = "mmiissiissiippii".to_string();
        let suffix_array = SuffixArray::<char, SaIs>::from_str(&s);
        for i in 0..s.len() - 1 {
            assert!(suffix_array.suffix_nth(i) < suffix_array.suffix_nth(i + 1));
        }
    }

    // #[test]
    // fn xor_shift() {
    //     input! {
    //         n: usize,
    //         a: [u32; n],
    //         b: [u32; n],
    //     }

    //     let a2 = a.repeat(2);
    //     let sa = SuffixArray::<SaIs>::new()
    // }
}
