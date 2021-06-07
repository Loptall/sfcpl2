#[cfg(not(feature = "atcoder"))]
mod dynamic;
#[cfg(not(feature = "atcoder"))]
pub use dynamic::Digit as DynamicDigit;

mod sta;
pub use sta::Digit as StaticDigit;

pub(crate) fn into(mut n: u64, d: u32) -> Vec<u32> {
    let mut res = Vec::new();

    while n > 0 {
        let rem = n % d as u64;
        res.push(rem as u32);
        n -= rem;
        n /= d as u64;
    }

    res.reverse();
    res
}
