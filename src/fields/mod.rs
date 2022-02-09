pub mod primefields;

use std::ops::{Add, AddAssign, Mul, Neg, Sub};

use num::{BigUint, Integer};
use rand::RngCore;

// This is actually need for the macro for tests to compile
#[allow(unused_imports)]
use paste::paste;

pub trait Field:
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
{
    fn zero() -> Self;
    fn one() -> Self;

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }

    fn invert(&self) -> Option<Self>;

    fn characteristic() -> BigUint;

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
        divide_and_conquer(self, i, Self::zero, Self::neg, Self::add)
    }

    fn square(&self) -> Self {
        self.pow(2u8)
    }

    fn pow(&self, i: impl Integer) -> Self {
        divide_and_conquer(&self, i, Self::one, |el| el.invert().unwrap(), Self::mul)
    }

    // Homomorphism Z -> F, Injective if restricted on Z_{char(F)}
    fn integer_embed(i: impl Integer) -> Self {
        Self::one().scale(i)
    }
}

// This is either double_and_add or square_and_multiply
fn divide_and_conquer<F, I, F1, F2, F3>(base: &F, i: I, id: F1, inv: F2, add: F3) -> F
where
    F: Clone,
    I: Integer,
    F1: FnOnce() -> F,
    F2: FnOnce(F) -> F,
    F3: Copy + Fn(F, F) -> F,
{
    if i.is_zero() {
        return id();
    }

    if i < I::zero() {
        return divide_and_conquer_impl(&inv(base.clone()), I::zero() - i, id, add);
    }

    divide_and_conquer_impl(base, i, id, add)
}

fn divide_and_conquer_impl<F, I, F1, F3>(base: &F, i: I, id: F1, add: F3) -> F
where
    F: Clone,
    I: Integer,
    F1: FnOnce() -> F,
    F3: Copy + Fn(F, F) -> F,
{
    if i.is_zero() {
        return id();
    }

    if i.is_one() {
        return base.clone();
    }

    let two = I::one() + I::one();

    if i.is_even() {
        let p = divide_and_conquer_impl(base, i / two, id, add);
        add(p.clone(), p)
    } else {
        let p = divide_and_conquer_impl(base, (i - I::one()) / two, id, add);
        add(base.clone(), add(p.clone(), p))
    }
}

#[macro_export]
macro_rules! field_tests {
    ($ff:ident) => {
        paste! {
            #[cfg(test)]
            mod [< $ff:snake _tests >] {
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
                    assert_eq!($ff::integer_embed(0), $ff::zero());
                    assert_eq!($ff::integer_embed(1), $ff::one());
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
                        assert_eq!(el.clone() - el.clone(), $ff::zero());

                        let el2 = $ff::random(&mut rng);
                        assert_eq!(el.clone() + el2.clone(), el2 + el);
                    }
                }

                #[test]
                fn multiplicative_identies() {
                    const NUM_ELEMENTS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    assert!($ff::zero().invert().is_none());
                    assert_eq!($ff::one() * $ff::one(), $ff::one());
                    for _ in 0..NUM_ELEMENTS {
                        let el = $ff::random_non_zero(&mut rng);
                        assert_eq!(el, $ff::one() * &el);
                        assert_eq!(el, el.clone() * $ff::one());
                        assert!(el.invert().is_some());
                        let inv = el.invert().unwrap();
                        assert_eq!(el.clone() * inv.clone(), $ff::one());
                        let el2 = $ff::random(&mut rng);
                        assert_eq!(el.clone() * el2.clone(), el2 * el);
                    }
                }

                #[test]
                fn distributivity() {
                    const NUM_ELEMENTS: usize = 1000;
                    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
                    for _ in 0..NUM_ELEMENTS {
                        let (a, b, c) = (
                            $ff::random(&mut rng),
                            $ff::random(&mut rng),
                            $ff::random(&mut rng),
                        );
                        assert_eq!(a.clone() * (b.clone() + c.clone()), a.clone() * b + a * c);
                    }
                }
            }
        }
    };
}
