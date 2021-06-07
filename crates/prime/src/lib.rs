pub mod atkin;
pub mod erathosthnes;

type Factors = Vec<(usize, usize)>;

pub trait Sieve {
    /// return whether self is prime with pre-initialized sieve
    fn is_prime(&self, x: usize) -> bool;
    fn primes(&mut self) -> Box<dyn Iterator<Item = usize>>;
    fn factorize(&mut self, mut x: usize) -> Factors {
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

pub trait Prime {
    /// return whether self is prime with trial_division method
    fn is_prime(self) -> bool;
    fn factorize(self) -> Factors;
}

impl<T: Into<usize>> Prime for T {
    fn is_prime(self) -> bool {
        trial_division(self.into())
    }

    fn factorize(self) -> Factors {
        factorize(self.into())
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

const M: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];

fn factorize(mut x: usize) -> Factors {
    let mut res = Vec::new();
    if x < 2 {
        return res;
    }
    let x2 = x;
    for &p in &[2, 3, 5] {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            res.push((p, e));
        }
    }

    'a: for i in 0.. {
        for &j in M.iter() {
            let p = 30 * i + j;
            dbg!(p);
            if p == 1 {
                continue;
            } else if p * p > x2 {
                if x > 1 {
                    res.push((x, 1));
                }
                break 'a;
            } else {
                let mut e = 0;
                while x % p == 0 {
                    x /= p;
                    e += 1;
                }
                if e > 0 {
                    res.push((p, e));
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::erathosthnes::Erathosthnes;
    use super::{Factors, Prime as _, Sieve as _};
    use rand::{thread_rng, Rng};

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

        assert!(1000000007usize.is_prime());
    }

    fn restore_factors(f: Factors) -> usize {
        f.into_iter()
            .map(|(p, e)| p.pow(e as u32))
            .product::<usize>()
    }

    #[test]
    fn factorize_basic() {
        assert_eq!(0usize.factorize(), vec![]);
        assert_eq!(1usize.factorize(), vec![]);
        assert_eq!(2usize.factorize(), vec![(2, 1)]);
        assert_eq!(3usize.factorize(), vec![(3, 1)]);
        assert_eq!(4usize.factorize(), vec![(2, 2)]);
        assert_eq!(5usize.factorize(), vec![(5, 1)]);
        assert_eq!(6usize.factorize(), vec![(2, 1), (3, 1)]);
        assert_eq!(7usize.factorize(), vec![(7, 1)]);
        assert_eq!(8usize.factorize(), vec![(2, 3)]);

        assert_eq!(57usize.factorize(), vec![(3, 1), (19, 1)]);
        assert_eq!((1usize << 30).factorize(), vec![(2, 30)]);
    }

    #[test]
    fn factorize_random() {
        let mut rng = thread_rng();
        for _ in 0..1000 {
            let r = rng.gen_range(2usize, 100000);
            let f = r.factorize();
            assert_eq!(r, restore_factors(f));
        }
    }

    #[test]
    fn factorize_with_sieve_basic() {
        let mut sieve = Erathosthnes::new(100);

        assert_eq!(sieve.factorize(0), vec![]);
        assert_eq!(sieve.factorize(1), vec![]);
        assert_eq!(sieve.factorize(2), vec![(2, 1)]);
        assert_eq!(sieve.factorize(3), vec![(3, 1)]);
        assert_eq!(sieve.factorize(4), vec![(2, 2)]);
        assert_eq!(sieve.factorize(5), vec![(5, 1)]);
        assert_eq!(sieve.factorize(6), vec![(2, 1), (3, 1)]);
        assert_eq!(sieve.factorize(7), vec![(7, 1)]);
        assert_eq!(sieve.factorize(8), vec![(2, 3)]);

        assert_eq!(sieve.factorize(57), vec![(3, 1), (19, 1)]);
        assert_eq!((1usize << 30).factorize(), vec![(2, 30)]);
    }

    #[test]
    fn factorize_with_sieve_random() {
        let mut rng = thread_rng();
        let mut sieve = Erathosthnes::new(100000);
        for _ in 0..1000 {
            let r = rng.gen_range(2usize, 100000);
            let f = sieve.factorize(r);
            assert_eq!(r, restore_factors(f));
        }
    }
}
