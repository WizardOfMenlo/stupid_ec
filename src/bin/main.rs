use num::{BigInt, BigUint};
use stupid_ec::{
    fields::{primefields::PrimeField4999, Field},
    gcd::egcd,
    poly::DensePolynomial,
};

fn main() {
    let a = BigUint::from(25u8);
    let n = BigUint::from(4999u32);
    let res = egcd(a.clone(), n.clone());

    dbg!(&res);

    if res.negative {
        dbg!(-BigInt::from(a * res.a_coeff) + BigInt::from(n * res.n_coeff));
    } else {
        dbg!(BigInt::from(a * res.a_coeff) - BigInt::from(n * res.n_coeff));
    }

    let el = PrimeField4999::new(BigUint::from(25u8));
    dbg!(PrimeField4999::one().scale(12));
    dbg!(el.clone().invert().unwrap());
    dbg!(el.clone() * el.invert().unwrap());

    println!(
        "{}",
        DensePolynomial::<PrimeField4999>::new_integers(vec![1, 2, 3, 4, 5])
    );

    let first = DensePolynomial::<PrimeField4999>::new_integers(vec![-4, 0, -2, 1]);
    let second = DensePolynomial::<PrimeField4999>::new_integers(vec![-3, 1]);
    let (q, r) = first.div_quotient_rem(&second);
    println!("q := {}, r := {}", q, r);
}
