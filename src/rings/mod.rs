pub mod integers_mod_ring;

use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

use num::{Integer, Unsigned};
use rand::RngCore;

// This is actually need for the macro for tests to compile
#[allow(unused_imports)]
use paste::paste;

use crate::double_and_add::{
    positive_double_and_add, possibly_negative_double_and_add, PositiveDoubleAndAddState,
    PossiblyNegativeDoubleAndAddState,
};

// Note, all our rings are commutative
pub trait Ring:
    Clone
    + PartialEq
    + Eq
    + Neg<Output = Self>
    + Add<Output = Self>
    + for<'a> Add<&'a Self, Output = Self>
    + Sub<Output = Self>
    + for<'a> Sub<&'a Self, Output = Self>
    + Mul<Output = Self>
    + for<'a> Mul<&'a Self, Output = Self>
    + AddAssign
    + MulAssign
{
    fn zero() -> Self;
    fn one() -> Self;

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }

    fn random(rng: &mut impl RngCore) -> Self;

    fn random_non_zero(rng: &mut impl RngCore) -> Self {
        loop {
            let sample = Self::random(rng);
            if !sample.is_zero() {
                return sample;
            }
        }
    }

    fn scale(&self, i: impl Integer) -> Self {
        let state = PossiblyNegativeDoubleAndAddState {
            base: self.clone(),
            operation: Self::add,
            identity: Self::zero,
            inversion: Self::neg,
        };

        possibly_negative_double_and_add(state, i)
    }

    fn square(&self) -> Self {
        self.positive_pow(2u8)
    }

    fn positive_pow(&self, i: impl Integer + Unsigned) -> Self {
        if self.is_zero() {
            return Self::zero();
        }

        let state = PositiveDoubleAndAddState {
            base: self.clone(),
            operation: Self::mul,
            identity: Self::one,
        };

        positive_double_and_add(state, i)
    }

    // Homomorphism Z -> F, Injective if restricted on Z_{char(F)}
    fn integer_embed(i: impl Integer) -> Self {
        Self::one().scale(i)
    }
}

#[macro_export]
macro_rules! ring_tests {
    ($rr:ident) => {
        paste! {
            #[cfg(test)]
            mod [< $rr:snake _ring_tests >] {
                use super::$rr;
                use crate::rings::Ring;
                use rand::SeedableRng;

                #[test]
                fn obvious_things() {
                    assert!($rr::zero().is_zero());
                    assert!($rr::one().is_one());
                    assert_eq!($rr::integer_embed(0), $rr::zero());
                    assert_eq!($rr::integer_embed(1), $rr::one());
                }

                #[test]
                fn additive_identies() {
                    const NUM_ELEMENTS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    assert_eq!($rr::zero() + $rr::zero(), $rr::zero());
                    for _ in 0..NUM_ELEMENTS {
                        let el = $rr::random(&mut rng);
                        assert_eq!(el, $rr::zero() + &el);
                        assert_eq!(el, el.clone() + $rr::zero());
                        assert_eq!(el.clone() - el.clone(), $rr::zero());

                        let el2 = $rr::random(&mut rng);
                        assert_eq!(el.clone() + el2.clone(), el2 + el);
                    }
                }

                #[test]
                fn multiplicative_identies() {
                    const NUM_ELEMENTS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    assert_eq!($rr::one() * $rr::one(), $rr::one());
                    for _ in 0..NUM_ELEMENTS {
                        let el = $rr::random_non_zero(&mut rng);
                        assert_eq!(el, $rr::one() * &el);
                        assert_eq!(el, el.clone() * $rr::one());

                        // NOTE: All our rings are commutative
                        let el2 = $rr::random(&mut rng);
                        assert_eq!(el.clone() * el2.clone(), el2 * el);
                    }
                }

                #[test]
                fn distributivity() {
                    const NUM_ELEMENTS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    for _ in 0..NUM_ELEMENTS {
                        let (a, b, c) = (
                            $rr::random(&mut rng),
                            $rr::random(&mut rng),
                            $rr::random(&mut rng),
                        );
                        assert_eq!(a.clone() * (b.clone() + c.clone()), a.clone() * b + a * c);
                    }
                }
            }
        }
    };
}
