use std::ops::{Add, Mul};

use lazy_static::lazy_static;
use num::{BigUint, Integer, One, Zero};
use paste::paste;

// Approach:
// 1. Write a Field Trait DONE
// 2. Write a PrimeField for some fixed moduli DONE
// 3. Make macro that generates a type for each modulo
// 4. Extend to polyfield

pub trait Field: Clone + PartialEq + Eq + Add<Output = Self> + Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;

    fn negate(self) -> Self;
    fn invert(self) -> Option<Self>;

    fn characteristic() -> BigUint;

    fn scale(self, i: impl Integer) -> Self {
        divide_and_conquer(self, i, Self::zero, Self::negate, Self::add)
    }
    fn pow(self, i: impl Integer) -> Self {
        divide_and_conquer(self, i, Self::one, |el| el.invert().unwrap(), Self::mul)
    }

    fn integer_embed(i: impl Integer) -> Self {
        Self::one().scale(i)
    }
}

// This is either double_and_add or square_and_multiply
fn divide_and_conquer<F, I, F1, F2, F3>(base: F, i: I, id: F1, inv: F2, add: F3) -> F
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
        return divide_and_conquer_impl(inv(base), i, id, add);
    }

    divide_and_conquer_impl(base, i, id, add)
}

fn divide_and_conquer_impl<F, I, F1, F3>(base: F, i: I, id: F1, add: F3) -> F
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
        return base;
    }

    let two = I::one() + I::one();

    if i.is_even() {
        let p = divide_and_conquer_impl(base, i / two, id, add);
        add(p.clone(), p)
    } else {
        let p = divide_and_conquer_impl(base.clone(), (i - I::one()) / two, id, add);
        add(base, add(p.clone(), p))
    }
}

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
        }

        impl Add for $ff {
            type Output = Self;
            fn add(self, rhs: Self) -> Self::Output {
                Self::new(self.el + rhs.el)
            }
        }

        impl Mul for $ff {
            type Output = Self;
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new(self.el * rhs.el)
            }
        }

        impl Field for $ff {
            fn zero() -> Self {
                Self::new(BigUint::zero())
            }
            fn one() -> Self {
                Self::new(BigUint::one())
            }

            fn negate(self) -> Self {
                Self::new(&*[<$ff:upper _MODULO>] - self.el)
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
