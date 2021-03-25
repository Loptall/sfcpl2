pub mod atkin;
pub mod erathosthnes;

pub trait SieveIsPrime<T> {
    /// return whether self is prime with pre-initialized sieve
    fn is_prime(&self, x: T) -> bool;
    fn primes(&mut self) -> Box<dyn Iterator<Item = usize>>;
}

impl<T: Into<usize>> SieveIsPrime<T> for atkin::Atkin {
    fn is_prime(&self, x: T) -> bool {
        self.is_prime(x.into())
    }

    fn primes(&mut self) -> Box<dyn Iterator<Item = usize>> {
        unimplemented!()
    }
}

impl<T: Into<usize>> SieveIsPrime<T> for erathosthnes::Erathosthnes {
    fn is_prime(&self, x: T) -> bool {
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
    use super::SelfIsPrime as _;
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
}
