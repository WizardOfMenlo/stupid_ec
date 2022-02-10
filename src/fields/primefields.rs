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

        crate::ring_generate!($ff, $mod);

        impl Field for $ff {
            fn invert(&self) -> Option<Self> {
                use crate::rings::Ring;
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
