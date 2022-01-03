use num::{bigint::RandBigInt, BigUint, Integer, One, Zero};

use crate::preconditions::{Checked, OddCheck};

pub type Odd<I> = Checked<I, OddCheck>;

// Write n = 2^s * d + 1, returns (s, d)
// Assumes that s < 2^64
pub fn rewrite_n<I: Integer>(n: Odd<I>) -> (usize, I) {
    let two = I::one() + I::one();
    let mut d = n.inner() - I::one();
    let mut s = 0;
    while d.is_multiple_of(&two) {
        d = d.div_ceil(&two);
        s += 1;
    }
    (s, d)
}

fn state_setup(n: BigUint) -> Result<InnerRabinState, MillerRabinResult> {
    if n == BigUint::zero() {
        return Err(MillerRabinResult::Zero);
    }
    if n == BigUint::one() {
        return Err(MillerRabinResult::One);
    }
    if n == BigUint::from(2 as u8) {
        return Err(MillerRabinResult::PossiblePrime);
    }

    let checked_n = Odd::try_new(n.clone());
    if checked_n.is_none() {
        return Err(MillerRabinResult::CompositeEven);
    }
    let state = InnerRabinState::new(checked_n.unwrap());
    Ok(state)
}

// Test if n is prime, using a as the witness, can only show probable primality
pub fn miller_rabin_step(n: BigUint, a: BigUint) -> MillerRabinResult {
    let poss_state = state_setup(n);
    if let Err(res) = poss_state {
        return res;
    }

    let state = poss_state.unwrap();

    assert!(a != state.n_1);
    inner_miller_rabin_step(&state, a)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MillerRabinResult {
    PossiblePrime,
    CompositeWitness(BigUint),
    CompositeEven,
    Zero,
    One,
}

impl MillerRabinResult {
    fn is_composite(&self) -> bool {
        match self {
            Self::PossiblePrime => false,
            _ => true,
        }
    }
}

pub fn miller_rabin(n: BigUint, rounds: usize) -> MillerRabinResult {
    let mut rng = rand::thread_rng();
    miller_rabin_with_randomness(&mut rng, n, rounds)
}

// In case not prime,  returns a witness
pub fn miller_rabin_with_randomness<R: rand::Rng>(
    rng: &mut R,
    n: BigUint,
    rounds: usize,
) -> MillerRabinResult {
    let poss_state = state_setup(n);
    if let Err(res) = poss_state {
        return res;
    }

    let state = poss_state.unwrap();

    for _ in 0..rounds {
        let a = rng.gen_biguint_range(&BigUint::from(2 as u8), &state.n);
        let partial_res = inner_miller_rabin_step(&state, a);
        if partial_res.is_composite() {
            return partial_res;
        }
    }

    MillerRabinResult::PossiblePrime
}

#[derive(Debug)]
struct InnerRabinState {
    n: BigUint,
    n_1: BigUint,
    s: usize,
    d: BigUint,
}

impl InnerRabinState {
    fn new(n: Odd<BigUint>) -> Self {
        let (s, d) = rewrite_n(n.clone());
        let n = n.inner();
        let n_1 = n.clone() - (1 as u8);
        InnerRabinState { n, n_1, s, d }
    }
}

fn inner_miller_rabin_step(state: &InnerRabinState, a: BigUint) -> MillerRabinResult {
    let mut x = a.modpow(&state.d, &state.n);
    if x == state.n_1 || x == BigUint::one() {
        return MillerRabinResult::PossiblePrime;
    }

    for _ in 1..state.s {
        x = x.modpow(&BigUint::from(2 as u8), &state.n);
        if x == state.n_1 {
            return MillerRabinResult::PossiblePrime;
        }
    }

    MillerRabinResult::CompositeWitness(a)
}

#[cfg(test)]
mod tests {
    use num::BigUint;
    use rand::SeedableRng;

    use crate::primes::{miller_rabin_with_randomness, rewrite_n, MillerRabinResult, Odd};

    #[test]
    fn representation_tests() {
        assert_eq!((0, 1), rewrite_n(Odd::new(1)));
        assert_eq!((2, 1), rewrite_n(Odd::new(4 + 1)));
        assert_eq!((5, 1), rewrite_n(Odd::new(32 + 1)));
        assert_eq!((5, 17), rewrite_n(Odd::new(32 * 17 + 1)));
    }

    #[test]
    fn primality_tests() {
        const ROUNDS: usize = 1000;
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        assert_eq!(
            miller_rabin_with_randomness(&mut rng, BigUint::from(0 as usize), ROUNDS),
            MillerRabinResult::Zero
        );
        assert_eq!(
            miller_rabin_with_randomness(&mut rng, BigUint::from(1 as usize), ROUNDS),
            MillerRabinResult::One
        );

        assert_eq!(
            miller_rabin_with_randomness(&mut rng, BigUint::from(4 as usize), ROUNDS),
            MillerRabinResult::CompositeEven
        );

        let primes : Vec<_> = [
            2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83,
            89, 97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179,
            181, 191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271,
            277, 281, 283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379,
            383, 389, 397, 401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479,
            487, 491, 499, 503, 509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599,
            601, 607, 613, 617, 619, 631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701,
            709, 719, 727, 733, 739, 743, 751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823,
            827, 829, 839, 853, 857, 859, 863, 877, 881, 883, 887, 907, 911, 919, 929, 937, 941,
            947, 953, 967, 971, 977, 983, 991, 997,
        ].iter().map(|s| BigUint::from(*s as usize)).collect();

        for i in primes {
            assert_eq!(
                miller_rabin_with_randomness(&mut rng, i.clone(), ROUNDS),
                MillerRabinResult::PossiblePrime,
                "Failure on {}", i
            )
        }
    }
}
