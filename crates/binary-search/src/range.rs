use std::ops::{Add, Div, RangeBounds, Sub};

use super::Average;
use util::ExpandRange;

const ZERO: usize = 0;
const LIM: usize = std::usize::MAX;

pub fn binary_search_by<T, R, F>(r: R, f: F) -> T
where
    T: ExpandRange + From<usize> + Average + Clone,
    R: RangeBounds<T>,
    F: Fn(&T) -> bool,
{
    let (mut left, mut right) = ExpandRange::expand_range(r, ZERO.into(), LIM.into());

    while right.clone() - left.clone() > 1usize.into() {
        let mid = Average::average(&left, &right);
        if f(&mid) {
            left = mid;
        } else {
            right = mid;
        }
    }

    left
}

#[cfg(test)]
mod tests {
    use super::binary_search_by;

    #[test]
    fn binary_search_basic() {
        assert_eq!(binary_search_by(0usize..10, |&x| x < 5), 4);
    }

    #[test]
    fn binary_search_large() {
        assert_eq!(binary_search_by(0usize..1000000000, |&x| x * x < 300), 17); // 17 * 17 = 289 < 300
        assert_eq!(binary_search_by(0usize.., |&x| x < 300), 299);
    }
}
