use std::ops::{Add, Mul};

use contracts::requires;
use lazy_static::lazy_static;
use num::{BigUint, Integer, One, Zero};

// Approach:
// 1. Write a Field Trait
// 2. Write a PrimeField for some fixed moduli
// 3. Make macro that generates a type for each modulo
// 4. Extend to polyfield

trait Field: Clone + PartialEq + Eq + Add<Output = Self> + Mul<Output = Self> {
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

lazy_static! {
    static ref MODULO: BigUint = BigUint::from(4999u32);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimeField {
    el: BigUint,
}

impl PrimeField {
    fn new(el: BigUint) -> Self {
        PrimeField { el: el % &*MODULO }
    }
}

impl Add for PrimeField {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        PrimeField::new(self.el + rhs.el)
    }
}

impl Mul for PrimeField {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        PrimeField::new(self.el * rhs.el)
    }
}

impl Field for PrimeField {
    fn zero() -> Self {
        PrimeField::new(BigUint::zero())
    }
    fn one() -> Self {
        PrimeField::new(BigUint::one())
    }

    fn negate(self) -> Self {
        PrimeField::new(&*MODULO - self.el)
    }

    fn invert(self) -> Option<Self> {
        if self.el.is_zero() {
            return None;
        }

        let res = extended_gcd(self.el.clone(), MODULO.clone());
        assert!(res.d.is_one());
        // x a - n b = 1
        if res.negative {
            Some(PrimeField::new(res.a_coeff))
        }
        // - x a + n b = 1
        else {
            Some(PrimeField::new(&*MODULO - res.a_coeff))
        }
    }

    fn characteristic() -> BigUint {
        return MODULO.clone();
    }
}

// TODO: Must optimize a lot
#[derive(Debug)]
pub struct GCDResult {
    pub d: BigUint,
    pub a_coeff: BigUint,
    pub n_coeff: BigUint,
    // False => -x a + n b = d
    // True => x a - n b = d
    pub negative: bool,
}

#[requires(a < n, "a must be smaller than n")]
pub fn extended_gcd(a: BigUint, n: BigUint) -> GCDResult {
    let mut qs = Vec::new();
    // TODO: Here as well, we only need the last two elements
    let mut rs = vec![n, a];

    loop {
        let curr_len = rs.len();
        let r_i = &rs[curr_len - 1];
        let r_i_1 = &rs[curr_len - 2];
        let (q_i, r_i_p_1) = r_i_1.div_mod_floor(r_i);
        if r_i_p_1.is_zero() {
            break;
        }
        qs.push(q_i);
        rs.push(r_i_p_1);
    }

    let ell = rs.len() - 1;

    // The gcd
    let d = rs[ell].clone();

    // TODO: In fact we don't need the whole vector, we can just use the last
    let mut cs = vec![BigUint::one()];
    let mut ds = vec![qs[ell - 2].clone()];

    for i in 1..=(ell - 2) {
        cs.push(ds[i - 1].clone());
        ds.push(cs[i - 1].clone() + ds[i - 1].clone() * qs[ell - 2 - i].clone());
    }

    GCDResult {
        d,
        a_coeff: ds.pop().unwrap(),
        n_coeff: cs.pop().unwrap(),
        negative: ell % 2 == 0,
    }
}
