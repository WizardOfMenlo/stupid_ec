pub mod primefields;

use num::{BigUint, Integer};

// This is actually need for the macro for tests to compile
#[allow(unused_imports)]
use paste::paste;

use crate::{
    double_and_add::{possibly_negative_double_and_add, PossiblyNegativeDoubleAndAddState},
    rings::Ring,
};

pub trait Field: Ring {
    fn invert(&self) -> Option<Self>;

    fn characteristic() -> BigUint;

    fn pow(&self, i: impl Integer) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let state = PossiblyNegativeDoubleAndAddState {
            base: self.clone(),
            operation: Self::mul,
            identity: Self::one,
            inversion: |el: Self| el.invert().unwrap(),
        };

        possibly_negative_double_and_add(state, i)
    }
}

#[macro_export]
macro_rules! field_tests {
    ($ff:ident) => {
        paste! {
            #[cfg(test)]
            mod [< $ff:snake _field_tests >] {
                use super::$ff;
                use crate::fields::Field;
                use crate::rings::Ring;
                use crate::primes::miller_rabin_with_randomness;
                use rand::SeedableRng;

                #[test]
                fn zero_is_not_one() {
                    assert!(!$ff::zero().is_one());
                    assert!(!$ff::one().is_zero());
                    assert!($ff::zero() != $ff::one());
                }

                #[test]
                fn char_is_prime() {
                    const ROUNDS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    assert!(
                        miller_rabin_with_randomness(&mut rng, $ff::characteristic(), ROUNDS)
                            .is_prime()
                    );
                }

                #[test]
                fn multiplicative_identies_with_inversions() {
                    const NUM_ELEMENTS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    assert!($ff::zero().invert().is_none());
                    for _ in 0..NUM_ELEMENTS {
                        let el = $ff::random_non_zero(&mut rng);
                        assert!(el.invert().is_some());
                        let inv = el.invert().unwrap();
                        assert_eq!(el.clone() * inv.clone(), $ff::one());
                    }
                }
            }
        }
    };
}
