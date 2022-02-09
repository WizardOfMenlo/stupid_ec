use std::ops::{Add, Mul, Neg, Sub};

use lazy_static::lazy_static;
use num::bigint::RandBigInt;
use num::{BigUint, Integer, One, Zero};
use paste::paste;
use rand::RngCore;

// Approach:
// 1. Write a Field Trait DONE
// 2. Write a PrimeField for some fixed moduli DONE
// 3. Make macro that generates a type for each modulo
// 4. Extend to polyfield

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
    };
}

field_generate!(PrimeField4999, BigUint::from(4999u32));
