/// convert a slice by run length encoding
///
/// return Vec<(T, usize)> where usize is continuous elements count
fn rle<T: Clone + PartialEq>(v: &[T]) -> Vec<(T, usize)> {
    let mut res = Vec::new();
    let mut count = 0;
    let mut last = None;
    for e in v {
        match last {
            None => {
                last = Some(e);
                count += 1;
            }
            Some(old) if old == e => {
                count += 1;
            }
            Some(_) => {
                res.push((last.unwrap().clone(), count));
                last = Some(e);
                count = 1;
            }
        }
    }
    res.push((last.unwrap().clone(), count));

    res
}

pub trait RunLengthEncoding<T> {
    fn rle(&self) -> Vec<(T, usize)>;
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &[T] {
    fn rle(&self) -> Vec<(T, usize)> {
        rle(self)
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for Vec<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        rle(self)
    }
}

impl<T: Clone + PartialEq> RunLengthEncoding<T> for &Vec<T> {
    fn rle(&self) -> Vec<(T, usize)> {
        rle(self)
    }
}

#[cfg(test)]
mod test {
    use super::RunLengthEncoding as _;
    #[test]
    fn test_rle() {
        let v = vec![1, 2, 2, 3, 3, 3];
        assert_eq!(v.rle(), vec![(1, 1), (2, 2), (3, 3)]);
    }
}
