use std::fmt::Display;

use super::into;

pub struct Digit<const BASE: u32> {
    raw_value: u64,
}

impl<const BASE: u32> Digit<BASE> {
    pub fn new(raw: u64) -> Self {
        Self { raw_value: raw }
    }

    pub fn raw(&self) -> u64 {
        self.raw_value
    }

    pub fn convert(&self) -> Vec<u32> {
        into(self.raw(), BASE)
    }
}

impl<const BASE: u32> Display for Digit<BASE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.convert();
        for c in v {
            if BASE <= 10 {
                write!(f, "{}", c)?
            } else {
                unimplemented!("how to display?")
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Digit;

    #[test]
    fn from_raw_test() {
        assert_eq!(Digit::<2>::new(18).convert(), vec![1, 0, 0, 1, 0]);
        assert_eq!(Digit::<8>::new(18).convert(), vec![2, 2]);
        assert_eq!(Digit::<10>::new(13).convert(), vec![1, 3]);
        assert_eq!(Digit::<16>::new(13).convert(), vec![13]);
    }

    #[test]
    fn stringify_test() {
        let octal = Digit::<8>::new(134);
        assert_eq!(octal.to_string(), "206");
    }
}
