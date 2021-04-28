use super::Average;

pub trait BinarySearch<T> {
    fn len(&self) -> usize;
    fn get(&self, i: usize) -> Option<&T>;
    fn binary_search_by<F: Fn(&T) -> bool>(&self, f: F) -> usize;
}

pub trait ExtBinarySearch<T: PartialOrd>: BinarySearch<T> {
    fn lower_bound(&self, v: &T) -> usize {
        self.binary_search_by(|x| x < v)
    }
    fn upper_bound(&self, v: &T) -> usize {
        self.binary_search_by(|x| x <= v)
    }
    fn count_lasting(&self, v: &T) -> usize {
        self.upper_bound(v) - self.lower_bound(v)
    }
    fn binary_search(&self, v: &T) -> Result<usize, usize> {
        let i = self.lower_bound(v);
        if let Some(val) = self.get(i) {
            if val == v {
                Ok(i)
            } else {
                Err(i)
            }
        } else {
            Err(i)
        }
    }
}

impl<T> BinarySearch<T> for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn get(&self, i: usize) -> Option<&T> {
        <[T]>::get(self, i)
    }

    fn binary_search_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        let mut left = 0;
        let mut right = self.len() - 1;

        while right - left > 1 {
            let mid = Average::average(&left, &right);
            if f(&self[mid]) {
                left = mid;
            } else {
                right = mid;
            }

            dbg!(left, right);
        }

        match (f(&self[left]), f(&self[right])) {
            (true, true) => self.len(),
            (true, false) => right,
            (false, true) => panic!("it maybe not sorted"),
            (false, false) => 0,
        }
    }
}

impl<T: Ord> ExtBinarySearch<T> for &[T] {}

impl<T> BinarySearch<T> for Vec<T> {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn get(&self, i: usize) -> Option<&T> {
        <[T]>::get(self, i)
    }

    fn binary_search_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        let mut left = 0;
        let mut right = self.len() - 1;

        while right - left > 1 {
            let mid = Average::average(&left, &right);
            if f(&self[mid]) {
                left = mid;
            } else {
                right = mid;
            }

            dbg!(left, right);
        }

        match (f(&self[left]), f(&self[right])) {
            (true, true) => self.len(),
            (true, false) => right,
            (false, true) => panic!("it maybe not sorted"),
            (false, false) => 0,
        }
    }
}

impl<T: Ord> ExtBinarySearch<T> for Vec<T> {}

impl<T> BinarySearch<T> for &Vec<T> {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }

    fn get(&self, i: usize) -> Option<&T> {
        <[T]>::get(self, i)
    }

    fn binary_search_by<F: Fn(&T) -> bool>(&self, f: F) -> usize {
        let mut left = 0;
        let mut right = self.len() - 1;

        while right - left > 1 {
            let mid = Average::average(&left, &right);
            if f(&self[mid]) {
                left = mid;
            } else {
                right = mid;
            }

            dbg!(left, right);
        }

        match (f(&self[left]), f(&self[right])) {
            (true, true) => self.len(),
            (true, false) => right,
            (false, true) => panic!("it maybe not sorted"),
            (false, false) => 0,
        }
    }
}

impl<T: Ord> ExtBinarySearch<T> for &Vec<T> {}

#[cfg(test)]
mod tests {
    use super::{BinarySearch, ExtBinarySearch};

    #[test]
    fn binary_search_by() {
        let v = vec![0, 1, 1, 1, 2, 2, 3];

        assert_eq!(v.binary_search_by(|&x| x < 0), 0);
        assert_eq!(v.binary_search_by(|&x| x < 1), 1);
        assert_eq!(v.binary_search_by(|&x| x < 2), 4);
        assert_eq!(v.binary_search_by(|&x| x < 3), 6);
        assert_eq!(v.binary_search_by(|&x| x < 4), 7);
    }

    #[test]
    fn binary_search() {
        let v = vec![0, 1, 1, 1, 2, 2, 3];

        assert_eq!(v.binary_search(&0), Ok(0));
        assert_eq!(v.binary_search(&1), Ok(1));
        assert_eq!(v.binary_search(&2), Ok(4));
        assert_eq!(v.binary_search(&3), Ok(6));
        assert_eq!(v.binary_search(&4), Err(7));
    }

    #[test]
    fn lower_bound() {
        let v = vec![0, 1, 1, 1, 2, 2, 3];

        assert_eq!(v.lower_bound(&0), 0);
        assert_eq!(v.lower_bound(&1), 1);
        assert_eq!(v.lower_bound(&2), 4);
        assert_eq!(v.lower_bound(&3), 6);
        assert_eq!(v.lower_bound(&4), 7);
    }

    #[test]
    fn upper_bound() {
        let v = vec![0, 1, 1, 1, 2, 2, 3];

        assert_eq!(v.upper_bound(&0), 1);
        assert_eq!(v.upper_bound(&1), 4);
        assert_eq!(v.upper_bound(&2), 6);
        assert_eq!(v.upper_bound(&3), 7);
        assert_eq!(v.upper_bound(&4), 7);
    }
}
