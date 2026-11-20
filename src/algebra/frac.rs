use std::ops::{Div, Mul};

#[allow(dead_code)]
const ONE_HALF: super::r64 = Frac(1, 2);
#[allow(dead_code)]
const ONE_THIRD: super::r64 = Frac(1, 3);

#[allow(non_camel_case_types)]
pub type r32 = Frac<i16>;
#[allow(non_camel_case_types)]
pub type r64 = Frac<i32>;
#[allow(non_camel_case_types)]
pub type r128 = Frac<i64>;

/// Fraction data type with support for hetereogeneous numerator and denomenator.
#[derive(Debug)]
pub struct Frac<N, D = N>(pub N, pub D);

impl<T: Div> From<Frac<T>> for f32
where
    f32: From<<T as Div>::Output>,
{
    fn from(value: Frac<T>) -> Self {
        f32::from(value.0 / value.1)
    }
}

impl<T: Div> From<Frac<T>> for f64
where
    f64: From<<T as Div>::Output>,
{
    fn from(value: Frac<T>) -> Self {
        f64::from(value.0 / value.1)
    }
}

impl<T: Mul> Mul for Frac<T, T> {
    type Output = Frac<<T as Mul>::Output>;

    fn mul(self, rhs: Self) -> Self::Output {
        Frac(self.0 * rhs.0, self.1 * rhs.1)
    }
}

impl<T: Mul> Div for Frac<T, T> {
    type Output = Frac<<T as Mul>::Output>;

    fn div(self, rhs: Self) -> Self::Output {
        Frac(self.0 * rhs.1, self.1 * rhs.0)
    }
}
