use num::{BigInt, BigUint};
use stupid_ec::field_new_impl::extended_gcd;

fn main() {
    let a = BigUint::from(7u8);
    let n = BigUint::from(17u8);
    let res = extended_gcd(a.clone(), n.clone());

    dbg!(&res);

    if res.negative {
        dbg!(-BigInt::from(a * res.a_coeff) + BigInt::from(n * res.n_coeff));
    } else {
        dbg!(BigInt::from(a * res.a_coeff) - BigInt::from(n * res.n_coeff));
    }
}
