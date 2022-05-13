use rand::RngCore;

use super::ops;

// TODO: Select the type depending on the radix
// Integers are represented as follows
// a_0 + a_1 * 2**64 + a_2 * 2**128 + ... = [a_0, a_1, ...]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FixedInteger<const LIMBS: usize>([u64; LIMBS]);

impl<const LIMBS: usize> FixedInteger<LIMBS> {
    pub const fn zero() -> Self {
        FixedInteger([0x0; LIMBS])
    }

    pub const fn one() -> Self {
        Self::from_u64(0x1)
    }

    pub const fn maxvalue() -> Self {
        FixedInteger([u64::MAX; LIMBS])
    }

    pub const fn from_u64(x: u64) -> Self {
        let mut arr = [0x0; LIMBS];
        arr[0] = x;
        FixedInteger(arr)
    }

    pub fn random(rng: &mut impl RngCore) -> Self {
        let mut arr = [0x0; LIMBS];
        for i in 0..LIMBS {
            arr[i] = rng.next_u64();
        }
        FixedInteger(arr)
    }
}

impl<const LIMBS: usize> std::ops::AddAssign for FixedInteger<LIMBS> {
    fn add_assign(&mut self, rhs: Self) {
        // We discard the carry
        ops::add_self(&mut self.0, &rhs.0);
    }
}

impl<'a, const LIMBS: usize> std::ops::AddAssign<&'a Self> for FixedInteger<LIMBS> {
    fn add_assign(&mut self, rhs: &Self) {
        // We discard the carry
        ops::add_self(&mut self.0, &rhs.0);
    }
}

impl<const LIMBS: usize> std::ops::Add for FixedInteger<LIMBS> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        let mut arr = [0x0; LIMBS];
        ops::add(&self.0, &rhs.0, &mut arr);
        FixedInteger(arr)
    }
}

impl<'a, const LIMBS: usize> std::ops::Add<&'a Self> for FixedInteger<LIMBS> {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        let mut arr = [0x0; LIMBS];
        ops::add(&self.0, &rhs.0, &mut arr);
        FixedInteger(arr)
    }
}

impl<const LIMBS: usize> std::ops::SubAssign for FixedInteger<LIMBS> {
    fn sub_assign(&mut self, rhs: Self) {
        // We discard the carry
        ops::sub_self(&mut self.0, &rhs.0);
    }
}

impl<'a, const LIMBS: usize> std::ops::SubAssign<&'a Self> for FixedInteger<LIMBS> {
    fn sub_assign(&mut self, rhs: &Self) {
        // We discard the carry
        ops::sub_self(&mut self.0, &rhs.0);
    }
}

impl<const LIMBS: usize>  std::ops::Sub for FixedInteger<LIMBS> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut arr = [0x0; LIMBS];
        ops::sub(&self.0, &rhs.0, &mut arr);
        FixedInteger(arr)
    }
}

impl<'a, const LIMBS: usize> std::ops::Sub<&'a Self> for FixedInteger<LIMBS> {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        let mut arr = [0x0; LIMBS];
        ops::sub(&self.0, &rhs.0, &mut arr);
        FixedInteger(arr)
    }
}

#[cfg(test)]
mod tests {
    use super::FixedInteger;
    use rand::SeedableRng;
    const ITERATIONS: usize = 1000;

    type IntType = FixedInteger<4>;

    #[test]
    fn zero_is_add_identity() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        for _ in 0..ITERATIONS {
            let el = IntType::random(&mut rng);
            assert_eq!(el, el + IntType::zero());
            assert_eq!(el, el - IntType::zero())
        }
    }

    #[test]
    fn addition_is_commutative() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        for _ in 0..ITERATIONS {
            let el1 = IntType::random(&mut rng);
            let el2 = IntType::random(&mut rng);
            assert_eq!(el1 + el2, el2 + el1)
        }
    }

    #[test]
    fn addition_is_associative() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        for _ in 0..ITERATIONS {
            let el1 = IntType::random(&mut rng);
            let el2 = IntType::random(&mut rng);
            let el3 = IntType::random(&mut rng);
            assert_eq!(el1 + (el2 + el3), (el1 + el2) + el3)
        }
    }

    #[test]
    fn addition_is_inverse_of_sub() {
        let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(42);
        for _ in 0..ITERATIONS {
            let el1 = IntType::random(&mut rng);
            let el2 = IntType::random(&mut rng);
            assert_eq!(el1 + el2 - el2, el1)
        }
    }
}
