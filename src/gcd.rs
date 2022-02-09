use contracts::requires;
use num::{BigUint, Integer, One, Zero};

// TODO: Must optimize a lot
#[derive(Debug)]
pub struct GCDResult {
    pub d: BigUint,
    // Coeff of the smaller of the two
    pub a_coeff: BigUint,

    // Coeff of the larger
    pub n_coeff: BigUint,
    // True => - x a + n b = d
    // False =>  x a - n b = d
    pub negative: bool,
}

pub fn egcd(a: BigUint, b: BigUint) -> GCDResult {
    if a <= b {
        egcd_impl(a, b)
    } else {
        egcd_impl(b, a)
    }
}

#[requires(a <= n, "a must be smaller than n")]
fn egcd_impl(a: BigUint, n: BigUint) -> GCDResult {
    if a.is_zero() {
        return GCDResult {
            d: n,
            a_coeff: BigUint::zero(),
            n_coeff: BigUint::one(),
            negative: true,
        };
    }

    if n.is_multiple_of(&a) {
        return GCDResult {
            d: a,
            a_coeff: BigUint::one(),
            n_coeff: BigUint::zero(),
            negative: false,
        };
    }

    egcd_typical(a, n)
}

#[requires(!a.is_zero())]
#[requires(a < n, "a must be smaller than n")]
#[requires(!n.is_multiple_of(&a))]
pub(crate) fn egcd_typical(a: BigUint, n: BigUint) -> GCDResult {
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

#[cfg(test)]
mod tests {
    use num::{BigInt, BigUint, Integer, Zero};

    use super::egcd;

    fn check_coefficients(mut a: BigUint, mut n: BigUint) {
        let res = egcd(a.clone(), n.clone());
        assert_eq!(a.gcd(&n), res.d);

        // FIXME: Currently a bug in num-integer makes this panics if d == 0
        if !res.d.is_zero() {
            assert!(a.is_multiple_of(&res.d));
            assert!(n.is_multiple_of(&res.d));
        }

        if a > n {
            (a, n) = (n, a);
        }

        let ax = BigInt::from(a * res.a_coeff);
        let ny = BigInt::from(n * res.n_coeff);
        let d = BigInt::from(res.d);

        if res.negative {
            assert_eq!(-ax + ny, d);
        } else {
            assert_eq!(ax - ny, d);
        }
    }

    #[test]
    fn test_gcd() {
        for a in 0..256u32 {
            for n in 0..256u32 {
                check_coefficients(BigUint::from(a), BigUint::from(n));
            }
        }
    }
}
