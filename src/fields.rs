pub use ff::{Field, PrimeField};

// One example field
#[derive(PrimeField)]
#[PrimeFieldModulus = "52435875175126190479447740508185965837690552500527637822603658699938581184513"]
#[PrimeFieldGenerator = "7"]
#[PrimeFieldReprEndianness = "little"]
struct Fp([u64; 4]);

// TODO: We might want this to be generic
pub fn integer_embed<F: Field>(n: isize) -> F {
    scale(n, F::one())
}

// Compute [n]x
pub fn scale<F: Field>(n: isize, x: F) -> F {
    if n < 0 {
        return scale(-n, -x);
    }

    if n == 0 {
        return F::zero();
    }

    scale_impl(n as usize, x)
}

fn scale_impl<F: Field>(n: usize, x: F) -> F {
    if n == 0 {
        return F::zero();
    }

    if n == 1 {
        return x;
    }

    if n % 2 == 0 {
        let p = scale_impl(n / 2, x);
        p + p
    } else {
        let p = scale_impl((n - 1) / 2, x);
        x + p + p
    }
}

// TODO: Apparently FF does not allow to work with characteristics ...

// Extend a prime field
