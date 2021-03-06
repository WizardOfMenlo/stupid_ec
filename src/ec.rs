use core::panic;

use crate::fields::Field;

// y^2 + a_1 x y + a_3 y = x^3 + a_2 x^2 + a_4 x + a_6
#[derive(Debug)]
pub struct GeneralForm<F> {
    a_1: F,
    a_2: F,
    a_3: F,
    a_4: F,
    a_6: F,
}

#[derive(Debug, Clone)]
pub enum Point<F> {
    Point((F, F)),
    Infinity,
}

macro_rules! accessor_impl {
    ($access:ident) => {
        fn $access(&self) -> F {
            self.$access.clone()
        }
    };
}

impl<F> GeneralForm<F>
where
    F: Field,
{
    accessor_impl!(a_1);
    accessor_impl!(a_2);
    accessor_impl!(a_3);
    accessor_impl!(a_4);
    accessor_impl!(a_6);

    // Utilities, refer to Silverman, arithmetic of elliptic curves, 2nd Ed, page 42
    fn b_2(&self) -> F {
        self.a_1.square() + self.a_4.scale(4)
    }

    fn b_4(&self) -> F {
        self.a_4.scale(2) + self.a_1() * self.a_3()
    }

    fn b_6(&self) -> F {
        self.a_3.square() + self.a_6.scale(4)
    }

    fn b_8(&self) -> F {
        self.a_1.square() * self.a_6() + (self.a_2() * self.a_6()).scale(4)
            - self.a_1() * self.a_3() * self.a_4()
            + self.a_2() * self.a_3.square()
            - self.a_4.square()
    }

    fn c_4(&self) -> F {
        self.b_2().square() - self.b_4().scale(24)
    }

    pub fn is_on_curve(&self, p: &Point<F>) -> bool {
        match p {
            Point::Infinity => true,
            Point::Point((x, y)) => {
                y.square() + self.a_1() * x * y + self.a_3() * y
                    == x.pow(3) + self.a_2() * x.square() + self.a_4() * x + self.a_6()
            }
        }
    }

    pub fn discriminant(&self) -> F {
        -self.b_2().square() * self.b_8()
            - (self.b_4().square() * self.b_4()).scale(8)
            - (self.b_6().square()).scale(27)
            + (self.b_2() * self.b_4() * self.b_6()).scale(9)
    }

    pub fn has_node(&self) -> bool {
        self.discriminant().is_zero() && !self.c_4().is_zero()
    }

    pub fn has_cusp(&self) -> bool {
        self.discriminant().is_zero() && self.c_4().is_zero()
    }

    pub fn j_invariant(&self) -> F {
        if self.discriminant().is_zero() {
            panic!("A curve of zero discriminant has no j-invariant")
        }

        let c_4_cube = self.c_4().square() * self.c_4();
        c_4_cube * self.discriminant().invert().unwrap()
    }

    // Nit, in fact here we can have any j in the algebraic completion of F, and
    // then get an elliptic curve in F(j), here instead we assume that j is an element of the field
    pub fn from_j_invariant(j: F) -> Self {
        if j.is_zero() {
            return GeneralForm {
                a_1: F::zero(),
                a_2: F::zero(),
                a_3: F::one(),
                a_4: F::zero(),
                a_6: F::zero(),
            };
        }

        if j == F::integer_embed(1728) {
            return GeneralForm {
                a_1: F::zero(),
                a_2: F::zero(),
                a_3: F::zero(),
                a_4: F::one(),
                a_6: F::zero(),
            };
        }

        let t = (j - F::integer_embed(1728)).invert().unwrap();

        let a_1 = F::one();
        let a_2 = F::zero();
        let a_3 = F::zero();
        let a_4 = t.scale(36);
        let a_6 = t;

        GeneralForm {
            a_1,
            a_2,
            a_3,
            a_4,
            a_6,
        }
    }

    pub fn negate(&self, p: &Point<F>) -> Point<F> {
        match p {
            Point::Infinity => Point::Infinity,
            Point::Point((x, y)) => {
                Point::Point((x.clone(), -y.clone() - self.a_1() * x - self.a_3()))
            }
        }
    }
}
