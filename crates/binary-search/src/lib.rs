use std::ops::{Add, Div, Sub};

pub mod range;
pub mod slice;

pub trait Average:
    Add<Output = Self> + Sub<Output = Self> + Div<Output = Self> + PartialOrd + Sized
{
    fn average(left: &Self, right: &Self) -> Self;
}

macro_rules! impl_average {
    ($t:ty) => {
        impl Average for $t {
            fn average(left: &Self, right: &Self) -> Self {
                assert!(left < right);
                left + (right - left) / 2
            }
        }
    };
}

impl_average!(usize);
impl_average!(u8);
impl_average!(u16);
impl_average!(u32);
impl_average!(u64);
impl_average!(u128);
impl_average!(i8);
impl_average!(i16);
impl_average!(i32);
impl_average!(i64);
impl_average!(i128);
