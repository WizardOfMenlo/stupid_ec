use super::Field;
use crate::field_tests;
use lazy_static::lazy_static;
use num::bigint::RandBigInt;
use num::{BigUint, One, Zero};
use paste::paste;
use rand::RngCore;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub};

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

        impl AddAssign for $ff {
            fn add_assign(&mut self, rhs: Self) {
                self.el += rhs.el;
                self.el %= &* [<$ff:upper _MODULO>];
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

        impl MulAssign for $ff {
            fn mul_assign(&mut self, rhs: Self) {
                self.el *= rhs.el;
                self.el %= &* [<$ff:upper _MODULO>];
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

        impl fmt::Display for $ff {

            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.el)
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

            fn invert(&self) -> Option<Self> {
                if self.el.is_zero() {
                    return None;
                }

                if self.el.is_one() {
                    return Some(Self::one());
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

        field_tests!($ff);
    };
}

field_generate!(PrimeField4999, BigUint::from(4999u32));
