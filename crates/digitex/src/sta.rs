use std::fmt::Display;

use super::into;

/// Generic number expression with variable base
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digit {
    raw_value: u64,
    base: u32,
}

impl Digit {
    pub fn new(n: u64, base: u32) -> Self {
        Self { raw_value: n, base }
    }

    pub fn raw(&self) -> u64 {
        self.raw_value
    }

    pub fn convert(&self) -> Vec<u32> {
        into(self.raw_value, self.base)
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = self.convert();
        for c in v {
            if c < 10 {
                write!(f, "{}", c)?
            } else {
                write!(f, "{}", ('A' as u8 + c as u8 - 10) as char)?
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Digit;

    #[test]
    fn from_raw_test() {
        assert_eq!(Digit::new(18, 2).convert(), vec![1, 0, 0, 1, 0]);
        assert_eq!(Digit::new(18, 8).convert(), vec![2, 2]);
        assert_eq!(Digit::new(13, 10).convert(), vec![1, 3]);
        assert_eq!(Digit::new(13, 16).convert(), vec![13]);
    }

    #[test]
    fn stringify_test() {
        let octal = Digit::new(134, 8);
        assert_eq!(octal.to_string(), "206");
    }
}
