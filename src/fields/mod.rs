pub mod primefields;

use std::ops::{Add, Mul, Neg, Sub};

use num::{BigUint, Integer};
use rand::RngCore;

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
{
    fn zero() -> Self;
    fn one() -> Self;

    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn is_one(&self) -> bool {
        self == &Self::one()
    }

    fn invert(self) -> Option<Self>;

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
        return divide_and_conquer_impl(&inv(base.clone()), i, id, add);
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
