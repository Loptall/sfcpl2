// match - find element wtih one requirement
// match_all - find all element wtih one requirement
// matches - find element wtih multi requirement
// matches_all - find all element wtih multi requirement

fn linear_match<T, U, F>(mut a: T, d: F) -> Option<U>
where
    T: Iterator<Item = U>,
    F: FnMut(&U) -> bool,
{
    a.find(d)
}

fn linear_matches<T, U, F>(a: T, d: F) -> Vec<U>
where
    T: Iterator<Item = U>,
    F: FnMut(&U) -> bool,
{
    a.filter(d).collect()
}

fn linear_match_all<T, U, F>(mut a: T, d: &[F]) -> Option<U>
where
    T: Iterator<Item = U>,
    F: Fn(&U) -> bool,
{
    a.find(|x| d.iter().all(|f| f(x)))
}

fn linear_matches_all<T, U, F>(a: T, d: &[F]) -> Vec<U>
where
    T: Iterator<Item = U>,
    F: Fn(&U) -> bool,
{
    a.filter(|x| d.iter().all(|f| f(x))).collect()
}

#[cfg(test)]
mod tests {
    use super::linear_match;

    fn l_match() {
        assert_eq!(linear_match(1..=3, |&x| x == 2), Some(2));
    }
}
