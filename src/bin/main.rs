use num::BigUint;
use stupid_ec::primes::{rewrite_n, Odd, miller_rabin};

fn main() {
    dbg!(rewrite_n(Odd::new(17)));
    dbg!(miller_rabin(BigUint::from(17 as u8), 10));
}