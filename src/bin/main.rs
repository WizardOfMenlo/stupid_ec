use num::{BigInt, BigUint};
use stupid_ec::{
    field_new_impl::{Field, PrimeField},
    gcd::extended_gcd,
};

fn main() {
    let a = BigUint::from(25u8);
    let n = BigUint::from(4999u32);
    let res = extended_gcd(a.clone(), n.clone());

    dbg!(&res);

    if res.negative {
        dbg!(-BigInt::from(a * res.a_coeff) + BigInt::from(n * res.n_coeff));
    } else {
        dbg!(BigInt::from(a * res.a_coeff) - BigInt::from(n * res.n_coeff));
    }

    let el = PrimeField::new(BigUint::from(25u8));
    dbg!(PrimeField::one().scale(12));
    dbg!(el.clone().invert().unwrap());
    dbg!(el.clone() * el.invert().unwrap());
}
