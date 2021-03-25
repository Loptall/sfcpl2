pub mod atkin;
pub mod erathosthnes;

pub trait Sieve {
    /// return whether self is prime with pre-initialized sieve
    fn is_prime(&self, x: usize) -> bool;
    fn primes(&mut self) -> Box<dyn Iterator<Item = usize>>;
    fn factorization(&mut self, mut x: usize) -> Vec<(usize, usize)> {
        let mut res = Vec::new();
        let x2 = x;
        for p in self.primes().take_while(|&p| p * p <= x2) {
            if x == 1 {
                break;
            }
            let mut e = 0;
            while x % p == 0 {
                e += 1;
                x /= p;
            }

            if e > 0 {
                res.push((p, e));
            }
        }

        if x > 1 {
            res.push((x, 1));
        }

        res
    }
}

impl Sieve for atkin::Atkin {
    fn is_prime(&self, x: usize) -> bool {
        self.is_prime(x.into())
    }

    fn primes(&mut self) -> Box<dyn Iterator<Item = usize>> {
        unimplemented!("primes() for atkin is not yet implemented")
    }
}

impl Sieve for erathosthnes::Erathosthnes {
    fn is_prime(&self, x: usize) -> bool {
        self.is_prime(x.into())
    }

    fn primes(&mut self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(<Self>::primes(self))
    }
}

pub trait SelfIsPrime {
    /// return whether self is prime with trial_division method
    fn is_prime(self) -> bool;
}

impl<T: Into<usize>> SelfIsPrime for T {
    fn is_prime(self) -> bool {
        trial_division(self.into())
    }
}

fn trial_division(x: usize) -> bool {
    match x {
        0 | 1 => false,
        2 => true,
        x => {
            for k in (2..).take_while(|&k| k * k <= x) {
                if x % k == 0 {
                    return false;
                }
            }
            true
        }
    }
}

#[cfg(test)]
mod test {
    use super::erathosthnes::Erathosthnes;
    use super::SelfIsPrime as _;
    use super::Sieve as _;
    #[test]
    fn trial_division() {
        assert!(!1usize.is_prime());
        assert!(2usize.is_prime());
        assert!(3usize.is_prime());
        assert!(!4usize.is_prime());
        assert!(5usize.is_prime());
        assert!(!6usize.is_prime());
        assert!(7usize.is_prime());
        assert!(!8usize.is_prime());
        assert!(!9usize.is_prime());
    }

    #[test]
    fn factorize() {
        let mut sieve = Erathosthnes::new(100);

        assert_eq!(sieve.factorization(0), vec![]);
        assert_eq!(sieve.factorization(1), vec![]);
        assert_eq!(sieve.factorization(2), vec![(2, 1)]);
        assert_eq!(sieve.factorization(3), vec![(3, 1)]);
        assert_eq!(sieve.factorization(4), vec![(2, 2)]);
        assert_eq!(sieve.factorization(5), vec![(5, 1)]);
        assert_eq!(sieve.factorization(6), vec![(2, 1), (3, 1)]);
        assert_eq!(sieve.factorization(7), vec![(7, 1)]);
        assert_eq!(sieve.factorization(8), vec![(2, 3)]);

        assert_eq!(sieve.factorization(57), vec![(3, 1), (19, 1)]);
    }
}
