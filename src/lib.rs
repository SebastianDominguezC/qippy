#[cfg(test)]
mod tests {
    use crate::StateVector;

    use num_complex::Complex32;
    #[test]
    fn check_addition() {
        let zero = StateVector::new_zero();
        let one = StateVector::new_one();

        let ones = zero + one;

        assert_eq!(
            ones,
            StateVector::new_state_vector(Complex32::from(1.0), Complex32::from(1.0))
        );
    }
}

use nalgebra::SMatrix;
use num_complex::Complex32;

use std::ops::Add;

pub type Qubit = SMatrix<Complex32, 2, 1>;

#[derive(Debug, PartialEq)]
pub struct StateVector(Qubit);

impl StateVector {
    pub fn new_state_vector(a: Complex32, b: Complex32) -> Self {
        let mut qubit: Qubit = SMatrix::default();
        qubit[(0, 0)] = a;
        qubit[(1, 0)] = b;
        Self(qubit)
    }

    pub fn empty() -> Self {
        Self::new_state_vector(Complex32::from(0.0), Complex32::from(0.0))
    }

    pub fn new_zero() -> Self {
        Self::new_state_vector(Complex32::from(1.0), Complex32::from(0.0))
    }

    pub fn new_one() -> Self {
        Self::new_state_vector(Complex32::from(0.0), Complex32::from(1.0))
    }

    pub fn new_plus() -> Self {
        let n = 1.0 / 2.0_f32.sqrt();
        let a = n * Complex32::from(1.0);
        Self::new_state_vector(a, a)
    }

    pub fn new_minus() -> Self {
        let n = 1.0 / 2.0_f32.sqrt();
        let a = n * Complex32::from(1.0);
        Self::new_state_vector(a, -a)
    }

    pub fn from_qubit(q: Qubit) -> Self {
        Self(q)
    }
}

impl Add for StateVector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let q = self.0 + other.0;
        Self::from_qubit(q)
    }
}
