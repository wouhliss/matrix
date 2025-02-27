use num_traits::{MulAdd, MulAddAssign, One};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait Field:
    Add<Output = Self>
    + AddAssign
    + Add
    + Sub<Output = Self>
    + SubAssign
    + Sub
    + Mul<Output = Self>
    + MulAssign
    + Mul
    + MulAdd<Output = Self>
    + MulAddAssign
    + MulAdd
    + Div<Output = Self>
    + DivAssign
    + Div
    + Neg<Output = Self>
    + PartialEq
    + PartialOrd
    + Copy
    + Default
    + One
    + Into<f64>
{
}

impl<
        T: Add<Output = Self>
            + AddAssign
            + Add
            + Sub<Output = Self>
            + SubAssign
            + Sub
            + Mul<Output = Self>
            + MulAssign
            + Mul
            + MulAdd<Output = Self>
            + MulAddAssign
            + MulAdd
            + Div<Output = Self>
            + DivAssign
            + Div
            + Neg<Output = Self>
            + PartialEq
            + PartialOrd
            + Copy
            + Default
            + One
            + Into<f64>,
    > Field for T
{
}
