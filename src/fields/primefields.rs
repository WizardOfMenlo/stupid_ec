use super::Field;
use lazy_static::lazy_static;
use num::bigint::RandBigInt;
use num::{BigUint, One, Zero};
use paste::paste;
use rand::RngCore;
use std::ops::{Add, Mul, Neg, Sub};

// MAKE SURE TO CALL THIS WITH A PRIME NUMBER!
#[macro_export]
macro_rules! field_generate {
    ($ff:ident, $mod:expr) => {
        paste! {
                    lazy_static! {
                        static ref [<$ff:upper _MODULO>] : BigUint = $mod;
                    }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $ff {
            el: BigUint,
        }

        impl $ff {
            pub fn new(el: BigUint) -> Self {
                Self { el: el % &*[<$ff:upper _MODULO>] }
            }

            // Use only when it is known to be in correct range
            fn new_unchecked(el: BigUint) -> Self {
                Self { el }
            }
        }

        impl Add for $ff {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                self + &rhs
            }
        }

        impl<'a>  Add<&'a Self> for $ff {
            type Output = Self;
            fn add(self, rhs: &'a Self) -> Self::Output {
                Self::new(self.el + &rhs.el)
            }
        }

        impl Sub for $ff {
            type Output = Self;
            fn sub(self, rhs: Self) -> Self::Output {
                self + (-rhs)
            }
        }

        impl<'a>  Sub<&'a Self> for $ff {
            type Output = Self;
            fn sub(self, rhs: &'a Self) -> Self::Output {
                self + (-rhs.clone())
            }
        }

        impl Mul for $ff {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                self * &rhs
            }
        }

        impl<'a>  Mul<&'a Self> for $ff {
            type Output = Self;
            fn mul(self, rhs: &'a Self) -> Self::Output {
                Self::new(self.el * &rhs.el)
            }
        }

        impl Neg for $ff {
            type Output = Self;
            fn neg(self) -> Self {
                if self.is_zero() {
                    return Self::zero();
                }
                Self::new_unchecked(&*[<$ff:upper _MODULO>] - self.el)
            }
        }

        impl Field for $ff {
            fn zero() -> Self {
                Self::new_unchecked(BigUint::zero())
            }
            fn one() -> Self {
                Self::new_unchecked(BigUint::one())
            }

            fn random(r: &mut impl RngCore) -> Self {
                Self::new_unchecked(r.gen_biguint_range(&BigUint::zero(), &*[<$ff:upper _MODULO>]))
            }

            fn invert(self) -> Option<Self> {
                if self.el.is_zero() {
                    return None;
                }

                let res = crate::gcd::egcd_typical(self.el.clone(), [<$ff:upper _MODULO>].clone());
                assert!(res.d.is_one());
                // -x a + n b
                if res.negative {
                    Some(Self::new(&*[<$ff:upper _MODULO>] - res.a_coeff))
                } else {
                    Some(Self::new(res.a_coeff))
                }
            }

            fn characteristic() -> BigUint {
                return [<$ff:upper _MODULO>].clone();
            }
        }
                }

        #[cfg(test)]
        mod tests {
            use super::$ff;
            use crate::fields::Field;
            use crate::primes::miller_rabin_with_randomness;
            use num::BigUint;
            use rand::SeedableRng;

            #[test]
            fn obvious_things() {
                assert!($ff::zero().is_zero());
                assert!($ff::one().is_one());
                assert!(!$ff::zero().is_one());
                assert!(!$ff::one().is_zero());
                assert!($ff::zero() != $ff::one());
                assert!($ff::characteristic() >= BigUint::from(2u8));
            }

            #[test]
            fn modulo_is_prime() {
                const ROUNDS: usize = 1000;
                let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                assert!(
                    miller_rabin_with_randomness(&mut rng, $ff::characteristic(), ROUNDS)
                        .is_prime()
                );
            }

            #[test]
            fn additive_identies() {
                const NUM_ELEMENTS: usize = 1000;
                let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                assert_eq!($ff::zero() + $ff::zero(), $ff::zero());
                for _ in 0..NUM_ELEMENTS {
                    let el = $ff::random(&mut rng);
                    assert_eq!(el, $ff::zero() + &el);
                    assert_eq!(el, el.clone() + $ff::zero());
                    assert_eq!(el.clone() - el, $ff::zero());
                }
            }
        }
    };
}

field_generate!(PrimeField4999, BigUint::from(4999u32));
