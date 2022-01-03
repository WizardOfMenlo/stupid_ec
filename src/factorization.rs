use std::collections::BTreeMap;

use num::{BigUint, Integer, One, Zero};

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

#[cfg(test)]
mod tests {
    use num::{bigint::RandBigInt, BigUint};
    use rand::SeedableRng;

    use crate::factorization::trial_factorization;

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
}
