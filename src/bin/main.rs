use num::BigUint;
use stupid_ec::primes::{miller_rabin, rewrite_n, Odd};

fn main() {
    dbg!(rewrite_n(Odd::new(17)));
    dbg!(miller_rabin(BigUint::from(17 as u8), 10));
}
