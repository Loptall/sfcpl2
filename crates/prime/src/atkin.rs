use bitset::BitSet;

/// Atkin's Sieve
///
/// search all primes in [0, n]
#[derive(Debug)]
pub struct Atkin {
    n: usize,
    table: BitSet,
    // pub primes: Vec<usize>,
}

impl Atkin {
    pub fn new(n: usize) -> Self {
        let mut table = BitSet::zeros(n + 1);
        for &z in &[1, 5] {
            for y in (z..).step_by(6).take_while(|x| x * x < n) {
                for x in 1.. {
                    let m = 4 * x * x + y * y;
                    if x * x >= n || m > n {
                        break;
                    }
                    table.flip(m);
                }
                for x in ((y + 1)..).step_by(2) {
                    let m = 3 * x * x - y * y;
                    if x * x >= n || m > n {
                        break;
                    }
                    table.flip(m);
                }
            }
        }
        for &z in &[2, 4] {
            for y in (z..).step_by(6).take_while(|x| x * x < n) {
                for x in (1..).step_by(2) {
                    let m = 3 * x * x + y * y;
                    if x * x >= n || m > n {
                        break;
                    }
                    table.flip(m);
                }
                for x in ((y + 1)..).step_by(2) {
                    let m = 3 * x * x - y * y;
                    if x * x >= n || m > n {
                        break;
                    }
                    table.flip(m);
                }
            }
        }
        for y in (3..).step_by(6).take_while(|x| x * x < n) {
            for &z in &[1, 2] {
                for x in (z..).step_by(3) {
                    let m = 4 * x * x + y * y;
                    if x * x >= n || m > n {
                        break;
                    }
                    table.flip(m);
                }
            }
        }

        for x in (5..).take_while(|x| x * x < n) {
            if table[x] {
                for y in (1..).map(|z| z * x * x).take_while(|&z| z < n) {
                    table.remove(y);
                }
            }
        }

        table.entry(2);
        table.entry(3);

        // eprintln!("{}", table);

        Self {
            n,
            table,
            // primes: ,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_prime(&self, x: usize) -> bool {
        assert!(x <= self.n);
        self.table[x]
    }

    pub fn primes(&self) -> Vec<usize> {
        (2..=self.len()).filter(|&x| self.is_prime(x)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Atkin;

    #[test]
    fn basic() {
        let sieve = Atkin::new(100);
        // dbg!(sieve.primes());
        // dbg!(&sieve);
        assert!(!sieve.is_prime(1));
        assert!(sieve.is_prime(2));
        assert!(sieve.is_prime(3));
        assert!(!sieve.is_prime(4));
        assert!(sieve.is_prime(5));
        assert!(!sieve.is_prime(6));
        assert!(sieve.is_prime(7));
        assert!(!sieve.is_prime(8));
        assert!(!sieve.is_prime(9));
    }
}
