use std::{fmt::Display, marker::PhantomData};

use super::into;

pub trait DigitBase {
    const BASE: u32;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bin;
impl DigitBase for Bin {
    const BASE: u32 = 2;
}
pub type Binary = Digit<Bin>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dec;
impl DigitBase for Dec {
    const BASE: u32 = 10;
}
pub type Decimal = Digit<Dec>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Oct;
impl DigitBase for Oct {
    const BASE: u32 = 8;
}
pub type Octal = Digit<Oct>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Hex;
impl DigitBase for Hex {
    const BASE: u32 = 16;
}
pub type Hexdecimal = Digit<Hex>;

/// Generic number expression with variable base
///
/// `D: DigitBase` is used for how the value is expressed.
///
/// Bases 2 - Binary, 8 - octal, 10 - decimal, 16 - hexdeximal are prepared by default
///
/// If you need custom bases, you can make it in three lines to add, like...
///
/// ```
/// use digitex::DigitBase;
///
/// struct Tri;
/// impl DigitBase for Tri {
///     const BASE: u32 = 3;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Digit<D: DigitBase> {
    raw_value: u64,
    phantom: PhantomData<fn() -> D>,
}

impl<D: DigitBase> Digit<D> {
    pub fn new(n: u64) -> Self {
        Self {
            raw_value: n,
            phantom: PhantomData,
        }
    }

    pub fn raw(&self) -> u64 {
        self.raw_value
    }

    pub fn convert(&self) -> Vec<u32> {
        into(self.raw_value, D::BASE)
    }
}


impl<D: DigitBase> Display for Digit<D> {
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
mod test {
    use super::{Binary, Decimal, Digit, DigitBase, Hexdecimal, Octal};

    #[test]
    fn from_raw_test() {
        assert_eq!(Binary::new(18).convert(), vec![1, 0, 0, 1, 0]);
        assert_eq!(Octal::new(18).convert(), vec![2, 2]);
        assert_eq!(Decimal::new(13).convert(), vec![1, 3]);
        assert_eq!(Hexdecimal::new(13).convert(), vec![13]);
    }

    #[test]
    fn custom_base_test() {
        struct Tri;
        impl DigitBase for Tri {
            const BASE: u32 = 3;
        }

        let n = Digit::<Tri>::new(18);
        assert_eq!(n.convert(), vec![2, 0, 0]);
    }

    #[test]
    fn stringify_test() {
        let octal = Octal::new(134);
        assert_eq!(octal.to_string(), "206");

        let hex = Hexdecimal::new(28);
        assert_eq!(hex.to_string(), "1C");
    }
}
