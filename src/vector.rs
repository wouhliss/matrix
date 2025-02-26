use std::fmt::{Display, Formatter, Result};

use crate::traits::Field;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector<K, const N: usize> {
    pub data: [K; N],
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

    pub fn add(&mut self, other: Self) {
        for i in 0..N {
            self.data[i] += other.data[i];
        }
    }

    pub fn sub(&mut self, other: Self) {
        for i in 0..N {
            self.data[i] -= other.data[i];
        }
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

    pub fn linear_combination(u: &[Self], coefs: &[K]) -> Self {
        let mut res = Vector::<K, N>::new(&[K::default(); N]);

        for i in 0..N {
            for (j, vec) in u.iter().enumerate() {
                res.data[i] = vec.data[i].mul_add(coefs[j], res.data[i]);
            }
        }

        res
    }

    pub fn lerp(u: Self, v: Self, t: K) -> Self {
        let mut res = Vector::<K, N>::new(&[K::default(); N]);

        for idx in 0..N {
            res.data[idx] = (v.data[idx] - u.data[idx]).mul_add(t, u.data[idx]);
        }

        res
    }

    pub fn angle_cos(u: &Self, v: &Self) -> f32 {
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
