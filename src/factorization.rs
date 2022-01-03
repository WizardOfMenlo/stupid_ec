use std::collections::BTreeMap;

use num::{bigint::RandBigInt, BigUint, Integer, One, Zero};
use rand::Rng;

use crate::primes::miller_rabin_with_randomness;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Factorization {
    map: BTreeMap<BigUint, u32>,
}

impl Factorization {
    pub fn new(v: impl IntoIterator<Item = (BigUint, u32)>) -> Self {
        let mut map = BTreeMap::new();
        for (div, mult) in v.into_iter() {
            if div == BigUint::zero() {
                panic!("Zero not allowed in factorisation");
            }
            if div == BigUint::one() {
                continue;
            }

            if map.contains_key(&div) {
                panic!("Duplicates not allowed")
            }

            map.insert(div, mult);
        }

        Factorization { map }
    }

    pub fn n(&self) -> BigUint {
        let mut start = BigUint::one();
        for (div, mult) in &self.map {
            start = start * div.pow(*mult)
        }
        start
    }

    pub fn merge(mut self, fact: Factorization) -> Self {
        for (div, mult) in fact.map {
            *self.map.entry(div).or_insert(0) += mult;
        }
        self
    }
}

pub fn trial_factorization(mut n: BigUint) -> Factorization {
    if n == BigUint::zero() {
        panic!("Zero not allowed");
    }

    if n == BigUint::one() {
        return Factorization::new(std::iter::empty());
    }

    let mut res = BTreeMap::new();
    let mut s = BigUint::from(2 as u8);
    while n != BigUint::one() {
        if n.is_multiple_of(&s) {
            *res.entry(s.clone()).or_insert(0) += 1;
            n = n / s.clone();
        } else {
            s = s + (1 as u8);
        }
    }

    Factorization::new(res)
}

#[allow(non_snake_case)]
pub fn pollard_rho_single_factor<R: Rng>(rng: &mut R, n: BigUint) -> Option<BigUint> {
    let s = rng.gen_biguint_range(&BigUint::zero(), &n);
    let b = rng.gen_biguint_range(&BigUint::one(), &(n.clone() - (2 as usize)));

    let mut A = s.clone();
    let mut B = s.clone();
    let mut g = BigUint::one();

    let two = BigUint::from(2 as u8);

    while g == BigUint::one() {
        A = (A.modpow(&two, &n) + b.clone()) % n.clone();
        let fB = (B.modpow(&two, &n) + b.clone()) % n.clone();
        B = (fB.modpow(&two, &n) + b.clone()) % n.clone();
        let diff = if A >= B {
            A.clone() - B.clone()
        } else {
            B.clone() - A.clone()
        };
        g = diff.gcd(&n);
    }

    if g >= n {
        None
    } else {
        Some(g)
    }
}

pub fn pollard_rho_single_factor_repeat<R: Rng>(
    rng: &mut R,
    n: BigUint,
    rounds: usize,
) -> Option<BigUint> {
    for _ in 0..rounds {
        let partial = pollard_rho_single_factor(rng, n.clone());
        if partial.is_some() {
            return partial;
        }
    }
    None
}

#[derive(Debug, Clone)]
pub struct PollardRhoParameters {
    pub trial_bound: BigUint,
    pub rho_rounds: usize,
    pub miller_rabin_rounds: usize,
}

pub fn pollard_rho_factorisation<R: Rng>(
    rng: &mut R,
    params: PollardRhoParameters,
    n: BigUint,
) -> Option<Factorization> {
    if n == BigUint::zero() {
        panic!("Zero not allowed");
    }
    if n == BigUint::one() {
        return Some(Factorization::new(std::iter::empty()));
    }

    if miller_rabin_with_randomness(rng, n.clone(), params.miller_rabin_rounds).is_prime() {
        return Some(Factorization::new(vec![(n, 1)]));
    }

    if n < params.trial_bound {
        return Some(trial_factorization(n));
    }

    let factor = pollard_rho_single_factor_repeat(rng, n.clone(), params.rho_rounds)?;
    let new_n = n / factor.clone();
    Some(
        pollard_rho_factorisation(rng, params, new_n)?
            .merge(Factorization::new(std::iter::once((factor, 1)))),
    )
}

#[cfg(test)]
mod tests {
    use num::{bigint::RandBigInt, BigUint, Integer};
    use rand::SeedableRng;

    use crate::factorization::{pollard_rho_single_factor, trial_factorization, pollard_rho_factorisation};

    use super::PollardRhoParameters;

    #[test]
    fn test_trial_factorization() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        for i in 1..100 {
            let n = BigUint::from(i as usize);
            let fact = trial_factorization(n.clone());
            assert_eq!(n, fact.n());
        }

        const ROUNDS: usize = 1000;
        for _ in 0..ROUNDS {
            let num =
                rng.gen_biguint_range(&BigUint::from(2 as u8), &BigUint::from(1000000 as usize));
            let fact = trial_factorization(num.clone());
            assert_eq!(fact.n(), num);
        }
    }

    #[test]
    fn test_pollard_rho_single_factor() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        const ROUNDS: usize = 1000;
        for _ in 0..ROUNDS {
            let num =
                rng.gen_biguint_range(&BigUint::from(2 as u8), &BigUint::from(1000000 as usize));
            let factor = pollard_rho_single_factor(&mut rng, num.clone());
            if let Some(fact) = factor {
                assert!(num.is_multiple_of(&fact));
            }
        }
    }

    #[test]
    fn test_pollard_rho_factorisation() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        let params = PollardRhoParameters {
            trial_bound: BigUint::from(1024u32),
            rho_rounds: 2048,
            miller_rabin_rounds: 1000
        };

        const ROUNDS: usize = 10;
        for _ in 0..ROUNDS {
            let num =
                rng.gen_biguint_range(&BigUint::from(1000000u32), &BigUint::from(10000000 as usize));
            let factor = pollard_rho_factorisation(&mut rng, params.clone(), num.clone());
            if let Some(fact) = factor {
                assert_eq!(fact.n(), num);
            }
        }
    }
}
