use std::fmt::{Display, Formatter, Result};
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
    + Div<Output = Self>
    + DivAssign
    + Div
    + Neg<Output = Self>
    + Into<f64>
    + PartialEq
    + Copy
    + Default
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
            + Div<Output = Self>
            + DivAssign
            + Div
            + Neg<Output = Self>
            + Into<f64>
            + PartialEq
            + Copy
            + Default,
    > Field for T
{
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<K, const N: usize> {
    data: [K; N],
}

impl<K: Field + Display, const N: usize> Display for Vector<K, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for item in self.data.iter() {
            writeln!(f, "[{:.1}]", item)?;
        }
        Ok(())
    }
}

impl<K: Field, const N: usize> Vector<K, N> {
    pub fn new(data: &[K; N]) -> Self {
        Vector { data: data.clone() }
    }

    pub fn add(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] += other.data[i];
        }
        result
    }

    pub fn sub(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] -= other.data[i];
        }
        result
    }

    pub fn mul(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] *= other.data[i];
        }
        result
    }

    pub fn div(self, other: Self) -> Self {
        let mut result = self;
        for i in 0..N {
            result.data[i] /= other.data[i];
        }
        result
    }

    pub fn scl(&mut self, a: K) {
        for num in self.data.iter_mut() {
            *num *= a;
        }
    }

    pub fn linear_combination(u: &[Vector<K, N>], coefs: &[K]) -> Vector<K, N> {
        let mut res: [K; N] = [K::default(); N];

        for (x, vec) in u.iter().enumerate() {
            for (y, num) in vec.data.iter().enumerate() {
                res[y] += *num * coefs[x];
            }
        }

        Vector::from(res)
    }
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(array: [K; N]) -> Self {
        Vector::new(&array)
    }
}

pub fn lerp<V: Field>(u: V, v: V, t: V) -> V {
    u + (v - u) * t
}

impl<K: Field, const N: usize> Vector<K, N> {
    pub fn lerp(u: Vector<K, N>, v: Vector<K, N>, t: K) -> Vector<K, N> {
        let mut res = [K::default(); N];

        for idx in 0..u.data.len() {
            res[idx] = u.data[idx] + (v.data[idx] - u.data[idx]) * t;
        }

        Vector::from(res)
    }
}

impl<K: Field, const N: usize> Matrix<K, N> {
    pub fn lerp(u: Matrix<K, N>, v: Matrix<K, N>, t: K) -> Matrix<K, N> {
        let mut res = [[K::default(); N]; N];

        for x in 0..u.data.len() {
            for y in 0..u.data[x].len() {
                res[x][y] = u.data[x][y] + (v.data[x][y] - u.data[x][y]) * t;
            }
        }

        Matrix::from(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<K, const N: usize> {
    data: [[K; N]; N],
}

impl<K: Field + Display, const N: usize> Display for Matrix<K, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in self.data.iter() {
            write!(f, "[")?;
            for (index, col) in row.iter().enumerate() {
                if index != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{:.1}", col)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}

impl<K: Field, const N: usize> Matrix<K, N> {
    pub fn new(data: [[K; N]; N]) -> Self {
        Matrix { data }
    }

    pub fn add(&mut self, v: Matrix<K, N>) {
        for i in 0..self.data.len() {
            if let Some(r1) = self.data.get_mut(i) {
                if let Some(r2) = v.data.get(i) {
                    for j in 0..r1.len() {
                        if let Some(x) = r1.get_mut(j) {
                            if let Some(y) = r2.get(j) {
                                *x = *x + *y;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn sub(&mut self, v: Matrix<K, N>) {
        for i in 0..self.data.len() {
            if let Some(r1) = self.data.get_mut(i) {
                if let Some(r2) = v.data.get(i) {
                    for j in 0..r1.len() {
                        if let Some(x) = r1.get_mut(j) {
                            if let Some(y) = r2.get(j) {
                                *x = *x - *y;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for row in self.data.iter_mut() {
            for col in row.iter_mut() {
                *col *= a;
            }
        }
    }
}

impl<K: Field, const N: usize> From<[[K; N]; N]> for Matrix<K, N> {
    fn from(array: [[K; N]; N]) -> Self {
        Matrix::new(array)
    }
}
