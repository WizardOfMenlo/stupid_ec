use std::{collections::HashMap, iter::FromIterator};

use num::Integer;

use crate::fields::Field;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

    pub fn new_integers(it: impl IntoIterator<Item = impl Integer>) -> Self {
        Self::new(it.into_iter().map(F::integer_embed))
    }

    pub fn new_degree_list_integers(
        degree_list: impl IntoIterator<Item = (usize, impl Integer)>,
    ) -> Self {
        Self::new_degree_list(
            degree_list
                .into_iter()
                .map(|(k, v)| (k, F::integer_embed(v))),
        )
    }

    pub fn new_degree_list(degree_list: impl IntoIterator<Item = (usize, F)>) -> Self {
        let degree_list: HashMap<_, _> = degree_list.into_iter().collect();

        let max_degree = degree_list
            .iter()
            .filter(|(_, v)| !v.is_zero())
            .map(|(k, _)| k)
            .max();

        if max_degree.is_none() || *max_degree.unwrap() == 0 {
            return Self::zero();
        }

        let max_degree = max_degree.unwrap();

        let mut backing = vec![F::zero(); max_degree + 1];
        for (k, v) in degree_list.iter().filter(|(_, v)| !v.is_zero()) {
            backing[*k] = v.clone();
        }

        Self::new(backing)
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

    pub fn leading(&self) -> F {
        self.degree().map(|d| self.coeff(d)).unwrap_or(F::zero())
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

    // Equivalent to multiplying by x^d
    pub fn shift(&self, d: usize) -> Self {
        Self::new(
            std::iter::repeat(F::zero())
                .take(d)
                .chain(self.coeff.iter().cloned()),
        )
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

    pub fn div_quotient_rem(&self, divisor: &DensePolynomial<F>) -> (Self, Self) {
        if divisor.is_zero() {
            panic!("Cannot reduce by the zero polynomial");
        }
        if self.is_zero() {
            return (Self::zero(), Self::zero());
        }

        let num_deg = self.coeff.len();
        let den_deg = divisor.coeff.len();

        let mut out: Vec<_> = self.coeff.iter().rev().cloned().collect();
        let divisor: Vec<_> = divisor.coeff.iter().rev().cloned().collect();

        let normalizer = divisor[0].invert().unwrap();
        for i in 0..(num_deg - den_deg + 1) {
            out[i] *= normalizer.clone();
            let coeff = out[i].clone();
            if !coeff.is_zero() {
                for j in 1..den_deg {
                    out[i + j] += -divisor[j].clone() * coeff.clone();
                }
            }
        }

        let out: Vec<_> = out.into_iter().rev().collect();
        let separator = den_deg - 1;

        (
            Self::new(out[separator..].iter().cloned()),
            Self::new(out[..separator].iter().cloned()),
        )
    }
}

impl<F> fmt::Display for DensePolynomial<F>
where
    F: Field + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            return write!(f, "{}", F::zero());
        }
        let n = self.degree().unwrap();
        for i in (0..=n).rev() {
            let coeff = self.coeff(i);
            if !coeff.is_zero() {
                write!(
                    f,
                    "{}{}{}",
                    if i == n { "" } else { " + " },
                    if !coeff.is_one() || i == 0 {
                        format!("{}", coeff)
                    } else {
                        String::new()
                    },
                    if i == 0 {
                        String::new()
                    } else if i == 1 {
                        " x".to_string()
                    } else {
                        format!(" x^{}", i)
                    }
                )?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::DensePolynomial;
    use crate::fields::primefields::PrimeField4999;
    use crate::rings::Ring;

    #[test]
    fn basic_construction() {
        let zero: DensePolynomial<PrimeField4999> = DensePolynomial::zero();
        assert!(zero.is_zero());
        assert!(zero.degree().is_none());
        assert!(zero.coeff(42).is_zero());
        let values = [1, 2, 3, 4, 5, 18, 0];
        let f: DensePolynomial<PrimeField4999> = DensePolynomial::new_integers(values);
        let g: DensePolynomial<PrimeField4999> = DensePolynomial::new_degree_list_integers(vec![
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 4),
            (4, 5),
            (5, 18),
        ]);
        for i in 0..values.len() {
            assert_eq!(f.coeff(i), PrimeField4999::integer_embed(values[i]));
            assert_eq!(g.coeff(i), PrimeField4999::integer_embed(values[i]));
        }

        assert_eq!(f.degree(), Some(5));
        assert_eq!(g.degree(), Some(5));
        assert_eq!(f, g);
    }

    #[test]
    fn evaluation() {
        // x^4 + 3 x^ 2 + 2 x + 1
        let f = DensePolynomial::new_integers(vec![1, 2, 3, 0, 1]);

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

    #[test]
    fn shift() {
        // x^4 + 1
        let f: DensePolynomial<PrimeField4999> =
            DensePolynomial::new_degree_list_integers(vec![(0, 1), (4, 1)]);

        // x^7 + x^3
        let g = DensePolynomial::new_degree_list_integers(vec![(3, 1), (7, 1)]);

        assert_eq!(f.shift(3), g);
    }
}
