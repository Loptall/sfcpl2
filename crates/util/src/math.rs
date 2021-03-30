/// return minimum `p` which applies `n <= 2 ^ p`
pub fn ceil_pow(n: usize) -> usize {
    n.next_power_of_two().trailing_zeros() as usize
}
