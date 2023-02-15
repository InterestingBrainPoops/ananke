use std::ops::{Add, Div, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vector<const N: usize> {
    inner: [f64; N],
}

impl<const N: usize> Vector<N> {
    pub fn from_array(array: [f64; N]) -> Vector<N> {
        Vector {
            inner: array.clone(),
        }
    }
    pub fn new(item: f64) -> Vector<N> {
        Vector { inner: [item; N] }
    }
    pub fn dot(self, rhs: Self) -> f64 {
        let mut out = 0.0;
        for (idx, item) in self.inner.iter().enumerate() {
            out += item * rhs.inner[idx];
        }
        out
    }
}

impl<const N: usize> Add<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];

        for (idx, item) in self.inner.iter().enumerate() {
            out[idx] = item + rhs.inner[idx];
        }

        Vector { inner: out }
    }
}
impl<const N: usize> Sub<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];

        for (idx, item) in self.inner.iter().enumerate() {
            out[idx] = item - rhs.inner[idx];
        }

        Vector { inner: out }
    }
}
impl<const N: usize> Div<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn div(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];

        for (idx, item) in self.inner.iter().enumerate() {
            out[idx] = item / rhs.inner[idx];
        }

        Vector { inner: out }
    }
}
impl<const N: usize> Mul<Vector<N>> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: Vector<N>) -> Self::Output {
        let mut out = [0.0; N];

        for (idx, item) in self.inner.iter().enumerate() {
            out[idx] = item * rhs.inner[idx];
        }

        Vector { inner: out }
    }
}
impl<const N: usize> Mul<f64> for Vector<N> {
    type Output = Vector<N>;

    fn mul(self, rhs: f64) -> Self::Output {
        let mut out = [0.0; N];

        for (idx, item) in self.inner.iter().enumerate() {
            out[idx] = item * rhs;
        }

        Vector { inner: out }
    }
}
