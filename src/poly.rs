use std::iter::FromIterator;

use crate::fields::Field;

#[derive(Debug, Clone)]
pub struct DensePolynomial<F> {
    coeff: Vec<F>,
}

impl<F> DensePolynomial<F>
where
    F: Field,
{
    pub fn new(it: impl IntoIterator<Item = F>) -> Self {
        let mut coeff: Vec<_> = it.into_iter().collect();
        // Remove trailing zeros
        loop {
            if coeff.is_empty() {
                break;
            }
            let last = coeff.pop().unwrap();
            if !last.is_zero() {
                coeff.push(last);
                break;
            }
        }
        Self { coeff }
    }

    // Use None to signify the zero polynomial (degree -\infty)
    pub fn degree(&self) -> Option<usize> {
        if self.coeff.len() == 0 {
            None
        } else {
            Some(self.coeff.len() - 1)
        }
    }

    pub fn zero() -> Self {
        Self { coeff: Vec::new() }
    }

    pub fn is_zero(&self) -> bool {
        self.degree().is_none()
    }

    pub fn coeff(&self, pos: usize) -> F {
        self.coeff.get(pos).cloned().unwrap_or(F::zero())
    }

    pub fn add(&self, other: &DensePolynomial<F>) -> Self {
        if self.is_zero() {
            return other.clone();
        }

        if other.is_zero() {
            return self.clone();
        }

        let deg = std::cmp::max(self.degree().unwrap(), other.degree().unwrap());
        Self::new(
            (0..=deg)
                .into_iter()
                .map(|i| self.coeff(i) + other.coeff(i)),
        )
    }

    pub fn negate(&self) -> Self {
        Self::new(self.coeff.iter().cloned().map(|a| -a))
    }

    pub fn evaluate(&self, x: F) -> F {
        if self.degree().is_none() {
            return F::zero();
        }

        // Horner method is nice :)
        let n = self.degree().unwrap();
        let mut b = self.coeff(n);
        for i in 1..=n {
            b = self.coeff(n - i) + b * x.clone();
        }
        return b;
    }

    pub fn mult(&self, other: &DensePolynomial<F>) -> Self {
        if self.is_zero() || other.is_zero() {
            return Self::zero();
        }
        let n = self.degree().unwrap();
        let m = other.degree().unwrap();

        let mut res = Vec::from_iter(std::iter::repeat(F::zero()).take(m + n));
        for i in 0..n {
            for j in 0..m {
                res[i + j] += self.coeff(i) * other.coeff(j);
            }
        }

        Self::new(res)
    }
}

#[cfg(test)]
mod tests {

    use super::DensePolynomial;
    use crate::fields::primefields::PrimeField4999;
    use crate::fields::Field;

    #[test]
    fn basic_construction() {
        let zero: DensePolynomial<PrimeField4999> = DensePolynomial::zero();
        assert!(zero.is_zero());
        assert!(zero.degree().is_none());
        assert!(zero.coeff(42).is_zero());
        let values = [1, 2, 3, 4, 5, 18, 0];
        let f = DensePolynomial::new(values.iter().copied().map(PrimeField4999::integer_embed));
        for i in 0..values.len() {
            assert_eq!(f.coeff(i), PrimeField4999::integer_embed(values[i]));
        }

        assert_eq!(f.degree(), Some(5));
    }

    #[test]
    fn evaluation() {
        // x^4 + 3 x^ 2 + 2 x + 1
        let f = DensePolynomial::new(vec![
            PrimeField4999::integer_embed(1),
            PrimeField4999::integer_embed(2),
            PrimeField4999::integer_embed(3),
            PrimeField4999::zero(),
            PrimeField4999::one(),
        ]);

        assert_eq!(f.evaluate(PrimeField4999::zero()), PrimeField4999::one());
        assert_eq!(
            f.evaluate(PrimeField4999::one()),
            PrimeField4999::integer_embed(7)
        );
        assert_eq!(
            f.evaluate(PrimeField4999::integer_embed(15)),
            PrimeField4999::integer_embed(1341)
        );
    }
}
