use std::fmt::{Display, Formatter, Result};

use crate::{traits::Field, vector::Vector};

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

    pub fn add(&mut self, other: Self) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] += other.data[i][j];
            }
        }
    }

    pub fn sub(&mut self, other: Self) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] -= other.data[i][j];
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for i in 0..M {
            for j in 0..N {
                self.data[i][j] *= a;
            }
        }
    }

    pub fn mul_vec(self, v: &Vector<K, N>) -> Vector<K, M> {
        let mut vec = Vector::<K, M>::new(&[K::default(); M]);

        for y in 0..M {
            for x in 0..N {
                vec.data_mut()[y] = self.data[y][x].mul_add(v.data()[x], vec.data()[y]);
            }
        }

        vec
    }

    pub fn mul_mat<const P: usize>(self, m: &Matrix<K, P, N>) -> Matrix<K, P, M> {
        let mut mat = Matrix::<K, P, M>::new(&[[K::default(); P]; M]);

        for y in 0..M {
            for x in 0..P {
                for z in 0..N {
                    mat.data[y][x] = self.data[y][z].mul_add(m.data[z][x], mat.data[y][x]);
                }
            }
        }

        mat
    }

    pub fn linear_combination(u: &[Self], coefs: &[K]) -> Self {
        let mut res = Self::new(&[[K::default(); N]; M]);

        for i in 0..M {
            for j in 0..N {
                for (idx, mat) in u.iter().enumerate() {
                    res.data[i][j] = mat.data[i][j].mul_add(coefs[idx], res.data[i][j]);
                }
            }
        }

        res
    }

    pub fn lerp(u: Self, v: Self, t: K) -> Self {
        let mut res = Self::new(&[[K::default(); N]; M]);

        for x in 0..M {
            for y in 0..N {
                res.data[x][y] = (v.data[x][y] - u.data[x][y]).mul_add(t, u.data[x][y]);
            }
        }

        res
    }

    pub fn transpose(&self) -> Matrix<K, M, N> {
        let mut res = Matrix::new(&[[K::default(); M]; N]);

        for y in 0..N {
            for x in 0..M {
                res.data[y][x] = self.data[x][y];
            }
        }

        res
    }

    pub fn row_echelon(&self) -> Self {
        Matrix::new(&[[K::default(); N]; M])
    }
}

impl<K: Field, const N: usize> Matrix<K, N, N> {
    pub fn trace(&self) -> K {
        let mut res = K::default();

        for i in 0..N {
            res += self.data[i][i];
        }

        res
    }
}

impl<K: Field, const N: usize, const M: usize> From<[[K; N]; M]> for Matrix<K, N, M> {
    fn from(array: [[K; N]; M]) -> Self {
        Matrix::new(&array)
    }
}
