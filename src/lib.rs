use num_traits::{MulAdd, MulAddAssign, One};
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
    + Into<f32>
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
            + Into<f32>,
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

    pub fn dot(&self, v: Self) -> K {
        let mut res = K::default();

        for (x, num) in self.data.iter().enumerate() {
            res = num.mul_add(v.data[x], res);
        }

        res
    }

    pub fn norm_1(self) -> K {
        let mut res = K::default();

        for x in self.data.iter() {
            res += if *x >= -*x { *x } else { -*x };
        }

        res
    }

    pub fn norm(self) -> f32 {
        let mut res = K::default();

        for x in self.data.iter() {
            res = x.mul_add(*x, res);
        }

        res.into().powf(0.5)
    }

    pub fn norm_inf(self) -> K {
        let mut res = if self.data[0] >= -self.data[0] {
            self.data[0]
        } else {
            -self.data[0]
        };

        for x in self.data.iter() {
            res = if *x >= -*x { *x } else { -*x }
        }

        res
    }

    pub fn linear_combination(u: &[Vector<K, N>], coefs: &[K]) -> Vector<K, N> {
        let mut res: [K; N] = [K::default(); N];

        for (x, vec) in u.iter().enumerate() {
            for (y, num) in vec.data.iter().enumerate() {
                res[y] = num.mul_add(coefs[x], res[y]);
            }
        }

        Vector::from(res)
    }

    pub fn angle_cos(u: &Vector<K, N>, v: &Vector<K, N>) -> f32 {
        u.dot(*v).into() / (u.norm() * v.norm())
    }
}

impl<K: Field> Vector<K, 3> {
    pub fn cross_product(u: &Vector<K, 3>, v: &Vector<K, 3>) -> Vector<K, 3> {
        Vector::<K, 3>::new(&[
            u.data[1] * v.data[2] - u.data[2] * v.data[1],
            u.data[2] * v.data[0] - u.data[0] * v.data[2],
            u.data[0] * v.data[1] - u.data[1] * v.data[0],
        ])
    }
}

impl<K: Field, const N: usize> From<[K; N]> for Vector<K, N> {
    fn from(array: [K; N]) -> Self {
        Vector::new(&array)
    }
}

pub fn lerp<V: Field>(u: V, v: V, t: V) -> V {
    (v - u).mul_add(t, u)
}

impl<K: Field, const N: usize> Vector<K, N> {
    pub fn lerp(u: Vector<K, N>, v: Vector<K, N>, t: K) -> Vector<K, N> {
        let mut res = [K::default(); N];

        for idx in 0..u.data.len() {
            res[idx] = (v.data[idx] - u.data[idx]).mul_add(t, u.data[idx]);
        }

        Vector::from(res)
    }
}

impl<K: Field, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn lerp(u: Matrix<K, N, M>, v: Matrix<K, N, M>, t: K) -> Matrix<K, N, M> {
        let mut res = [[K::default(); N]; M];

        for x in 0..u.data.len() {
            for y in 0..u.data[x].len() {
                res[x][y] = (v.data[x][y] - u.data[x][y]).mul_add(t, u.data[x][y]);
            }
        }

        Matrix::from(res)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix<K, const N: usize, const M: usize> {
    data: [[K; N]; M],
}

impl<K: Field + Display, const N: usize, const M: usize> Display for Matrix<K, N, M> {
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

impl<K: Field, const N: usize, const M: usize> Matrix<K, N, M> {
    pub fn new(data: &[[K; N]; M]) -> Self {
        Matrix { data: data.clone() }
    }

    pub fn add(&mut self, v: Matrix<K, N, M>) {
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

    pub fn sub(&mut self, v: Matrix<K, N, M>) {
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

    pub fn mul_vec(self, v: &Vector<K, N>) -> Vector<K, M> {
        let mut vec = Vector::<K, M>::new(&[K::default(); M]);

        for y in 0..M {
            for x in 0..N {
                vec.data[y] = self.data[y][x].mul_add(v.data[x], vec.data[y]);
            }
        }

        vec
    }

    pub fn mul_mat(self, m: &Matrix<K, M, N>) -> Matrix<K, N, M> {
        let mut mat = Matrix::<K, N, M>::new(&[[K::default(); N]; M]);

        for y in 0..M {
            for x in 0..N {
                for z in 0..N {
                    mat.data[y][x] = self.data[y][z].mul_add(m.data[z][x], mat.data[y][x]);
                }
            }
        }

        mat
    }
}

impl<K: Field, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K, N, M> {
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(&array)
    }
}
