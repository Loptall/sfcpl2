use bitset::BitSet;

pub struct Erathosthnes {
    n: usize,
    table: BitSet,
    primes: Vec<usize>,
}

impl Erathosthnes {
    pub fn new(n: usize) -> Self {
        // table[i] = if 2 * i + 1 is prime
        let n = (n + 1) / 2;
        let mut table = BitSet::ones(n);
        let mut primes = Vec::new();
        table.remove(0);

        for i in (1..).take_while(|&x| x * x < n) {
            if table[i] {
                primes.push(2 * i + 1);
                for j in (2 * i * (i + 1)..)
                    .step_by(2 * i + 1)
                    .take_while(|&x| x < n)
                {
                    table.remove(j);
                }
            }
        }

        Self { n, table, primes }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_prime(&self, x: usize) -> bool {
        if x & 1 != 0 {
            self.table[x / 2]
        } else {
            x == 2
        }
    }

    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.primes.iter()
    }
}

#[cfg(test)]
mod test {
    use super::Erathosthnes;

    #[test]
    fn basic() {
        let sieve = Erathosthnes::new(10);
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
