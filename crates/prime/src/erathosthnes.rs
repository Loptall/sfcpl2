use std::rc::Rc;

use bitset::BitSet;

pub struct Erathosthnes {
    n: usize,
    table: BitSet,
    primes: Option<Vec<usize>>,
}

impl Erathosthnes {
    pub fn new(n: usize) -> Self {
        // table[i] = if 2 * i + 1 is prime
        let n = dbg!((n + 1) / 2);
        let mut table = dbg!(BitSet::ones(n));
        // let mut primes = Vec::new();
        table.remove(0);
        // primes.push(2);

        for i in (1..).take_while(|&x| x * x < n) {
            if table[i] {
                // primes.push(2 * i + 1);
                for j in (2 * i * (i + 1)..)
                    .step_by(2 * i + 1)
                    .take_while(|&x| x < n)
                {
                    table.remove(j);
                }
            }
        }

        Self {
            n,
            table,
            primes: None,
        }
    }

    pub fn len(&self) -> usize {
        self.n
    }

    pub fn is_prime(&self, x: usize) -> bool {
        if x % 2 == 1 {
            self.table[x / 2]
        } else {
            x == 2
        }
    }

    pub fn primes(&mut self) -> Primes {
        if self.primes.is_none() {
            self.primes = Some(self.table.iter().map(|x| 2 * x + 1).collect());
        }
        Primes {
            inner: Rc::new(self.primes.clone().unwrap()),
            two: false,
            cur: 0,
        }
    }
}

pub struct Primes {
    inner: Rc<Vec<usize>>,
    two: bool,
    cur: usize,
}

impl<'a> Iterator for Primes {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.two {
            let res = self.inner.get(self.cur).cloned();
            self.cur += 1;
            res
        } else {
            self.two = true;
            Some(2)
        }
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

    #[test]
    fn iter() {
        let mut sieve = Erathosthnes::new(10);
        assert_eq!(sieve.primes().collect::<Vec<usize>>(), vec![2, 3, 5, 7]);
        assert_eq!(sieve.primes().collect::<Vec<usize>>(), vec![2, 3, 5, 7]);
        assert_eq!(sieve.primes().collect::<Vec<usize>>(), vec![2, 3, 5, 7]);
    }
}
