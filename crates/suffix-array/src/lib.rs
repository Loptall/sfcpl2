use std::{collections::BTreeMap, marker::PhantomData};

use unique_count::UniqueCount as _;
use util::Shuffle as _;

pub trait Sort {
    fn sort(s: &str) -> Vec<&str>;
}

#[derive(Debug)]
struct DefaultSort;
impl Sort for DefaultSort {
    fn sort(s: &str) -> Vec<&str> {
        let mut inner = Vec::new();
        for i in 0..s.len() {
            inner.push(&s[i..]);
        }

        inner.sort();
        inner
    }
}

#[derive(Debug)]
struct SaIs;
impl Sort for SaIs {
    fn sort(s: &str) -> Vec<&str> {
        fn induced_sort(s: &[char], types: &[bool], seed: Vec<usize>) -> Vec<usize> {
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
            let mut inserted_lms = BTreeMap::<char, usize>::new();
            for i in seed {
                res[bin[&s[i]].1 - 1 - {
                    let h = inserted_lms.entry(s[i]).or_default();
                    let r = *h;
                    *h += 1;
                    r
                }] = Some(i);
            }

            // find L
            let mut inserted_l = BTreeMap::<char, usize>::new();
            for i in 0..n {
                if let Some(x) = res[i] {
                    if x > 0 && is_l(&(x - 1)) {
                        res[bin[&s[x - 1]].0 + {
                            let h = inserted_l.entry(s[x - 1]).or_default();
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
            let mut inserted_s = BTreeMap::<char, usize>::new();
            for i in (0..n).rev() {
                if let Some(x) = res[i] {
                    if x > 0 && is_s(&(x - 1)) {
                        res[bin[&s[x - 1]].1 - 1 - {
                            let h = inserted_s.entry(s[x - 1]).or_default();
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

        let raw = s;
        let s = format!("{}$", s).chars().collect::<Vec<char>>();
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

        let is_lms = |x: &usize| *x > 0 && types[*x] == S && types[*x - 1] == L;

        // temporary seed
        let seed = (1..n).filter(is_lms).collect::<Vec<_>>().shuffle();

        // first sort
        eprintln!("first sort");
        let res = induced_sort(&s, &types, seed);

        // let lms = (0..n).filter(is_lms).collect::<Vec<_>>();
        let lms = res
            .iter()
            .filter(|i| is_lms(i))
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
        let seed = if count + 1 >= idx.len() {
            let mut r = vec![0; m];
            for (i, c) in idx.into_iter().enumerate() {
                r[c] = i;
            }
            r.into_iter().map(|x| res[x]).collect()
        } else {
            let idx = &idx
                .into_iter()
                .map(|x| ('a' as u8 + x as u8) as char)
                .collect::<String>();
            SaIs::sort(idx)
                .into_iter()
                .map(|x| lms[idx.len() - x.len()])
                .collect()
        };

        // second sort
        eprintln!("second sort");
        let res = induced_sort(&s, &types, seed);

        res.into_iter()
            .map(|x| &raw[x..])
            .filter(|&x| !x.is_empty())
            .collect()
    }
}

/// String index
#[derive(Debug)]
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
        Self {
            phantom: PhantomData,
            raw: s,
            inner: S::sort(s),
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

    /// Represents suffix array in its suffix's start indices
    pub fn suffix_array(&self) -> Vec<usize> {
        self.inner.iter().map(|&x| self.len() - x.len()).collect()
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
    use super::{DefaultSort, SaIs, SuffixArray};
    #[test]
    fn basic() {
        let s = "abracadabra";
        let suffix_array = SuffixArray::<DefaultSort>::new(s);
        assert_eq!(suffix_array.find("br"), vec![8, 1]);
        assert_eq!(suffix_array.find("abra"), vec![7, 0]);
    }

    #[test]
    fn sa_is() {
        let s = "mmiissiissiippii";
        let suffix_array = SuffixArray::<SaIs>::new(s);
        for i in 0..s.len() - 1 {
            assert!(suffix_array.inner[i] < suffix_array.inner[i + 1]);
        }
    }
}
