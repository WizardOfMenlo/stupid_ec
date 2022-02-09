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
}
