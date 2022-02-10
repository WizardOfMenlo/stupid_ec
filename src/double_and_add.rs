use num::{Integer, Unsigned};

#[derive(Debug, Clone)]
pub(crate) struct PositiveDoubleAndAddState<R, F1, F2> {
    pub(crate) base: R,
    pub(crate) operation: F1,
    pub(crate) identity: F2,
}

pub(crate) fn positive_double_and_add<R, F1, F2, I>(
    state: PositiveDoubleAndAddState<R, F1, F2>,
    mut exponent: I,
) -> R
where
    R: Clone,
    I: Unsigned + Integer,
    F1: Fn(R, R) -> R,
    F2: FnOnce() -> R,
{
    if exponent.is_zero() {
        return (state.identity)();
    }

    let two = I::one() + I::one();
    let mut base = state.base.clone();
    while exponent.is_even() {
        base = (state.operation)(base.clone(), base);
        exponent = exponent.div_floor(&two);
    }

    if exponent.is_one() {
        return base;
    }

    let mut acc = base.clone();
    while exponent > I::one() {
        exponent = exponent.div_floor(&two);
        base = (state.operation)(base.clone(), base);
        if exponent.is_odd() {
            acc = (state.operation)(acc, base.clone());
        }
    }

    acc
}

#[derive(Debug, Clone)]
pub(crate) struct PossiblyNegativeDoubleAndAddState<R, F1, F2, F3> {
    pub(crate) base: R,
    pub(crate) operation: F1,
    pub(crate) identity: F2,
    pub(crate) inversion: F3,
}

pub(crate) fn possibly_negative_double_and_add<R, F1, F2, F3, I>(
    mut state: PossiblyNegativeDoubleAndAddState<R, F1, F2, F3>,
    mut exponent: I,
) -> R
where
    R: Clone,
    I: Integer,
    F1: Fn(R, R) -> R,
    F2: FnOnce() -> R,
    F3: FnOnce(R) -> R,
{
    if exponent.is_zero() {
        return (state.identity)();
    }

    if exponent < I::zero() {
        state.base = (state.inversion)(state.base);
        exponent = I::zero() - exponent;
    }

    let two = I::one() + I::one();
    let mut base = state.base.clone();
    while exponent.is_even() {
        base = (state.operation)(base.clone(), base);
        exponent = exponent.div_floor(&two);
    }

    if exponent.is_one() {
        return base;
    }

    let mut acc = base.clone();
    while exponent > I::one() {
        exponent = exponent.div_floor(&two);
        base = (state.operation)(base.clone(), base);
        if exponent.is_odd() {
            acc = (state.operation)(acc, base.clone());
        }
    }

    acc
}

#[cfg(test)]
mod tests {
    use crate::double_and_add::positive_double_and_add;

    use super::PositiveDoubleAndAddState;
    use num::{BigUint, One, Zero};

    #[test]
    fn test_scale() {
        let a = BigUint::from(10u8);
        let state = PositiveDoubleAndAddState {
            base: a.clone(),
            operation: |a: BigUint, b: BigUint| a + b,
            identity: BigUint::zero,
        };
        for i in 0..1024u32 {
            assert_eq!(a.clone() * i, positive_double_and_add(state.clone(), i));
        }
    }

    #[test]
    fn test_pow() {
        let a = BigUint::from(10u8);
        let state = PositiveDoubleAndAddState {
            base: a.clone(),
            operation: |a: BigUint, b: BigUint| a * b,
            identity: BigUint::one,
        };
        for i in 0..10 {
            assert_eq!(a.pow(i), positive_double_and_add(state.clone(), i));
        }
    }
}
