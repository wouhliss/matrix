use crate::traits::Field;

pub mod matrix;
pub mod traits;
pub mod vector;

pub fn lerp<V: Field>(u: V, v: V, t: V) -> V {
    (v - u).mul_add(t, u)
}
