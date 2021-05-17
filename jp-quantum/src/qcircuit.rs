use crate::vector::{Complex, Vector};
use std::fmt;
use std::ops::{Add, Mul};

/// A square matrix of size (2^n x 2^n) of complex numbers.
#[derive(Debug, Clone)]
pub struct QCircuit {
    size: usize,
    matrix: nalgebra::DMatrix<Complex>,
}

impl fmt::Display for QCircuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.matrix)
    }
}

impl QCircuit {
    pub fn empty() -> QCircuit {
        QCircuit {
            size: 0,
            matrix: nalgebra::DMatrix::from_row_slice(1, 1, &[Complex::new(1.0, 0.0)]),
        }
    }

    pub fn one_qbit_gate(matrix: &[Complex; 4]) -> QCircuit {
        QCircuit {
            size: 1,
            matrix: nalgebra::DMatrix::from_row_slice(2, 2, matrix),
        }
    }

    pub fn two_qbit_gate(matrix: &[Complex; 16]) -> QCircuit {
        QCircuit {
            size: 2,
            matrix: nalgebra::DMatrix::from_row_slice(4, 4, matrix),
        }
    }

    pub fn three_qbit_gate(matrix: &[Complex; 64]) -> QCircuit {
        QCircuit {
            size: 3,
            matrix: nalgebra::DMatrix::from_row_slice(8, 8, matrix),
        }
    }

    pub fn tensor_product(self, other: QCircuit) -> QCircuit {
        QCircuit {
            size: self.size + other.size,
            matrix: self.matrix.kronecker(&other.matrix),
        }
    }

    pub fn compose(self, other: QCircuit) -> QCircuit {
        assert_eq!(
            self.size, other.size,
            "circuits can only be composed if they're the same size"
        );
        QCircuit {
            size: self.size,
            // Backwards?
            matrix: other.matrix * self.matrix,
        }
    }

    pub fn run(self) -> Vector {
        let input = Vector::zero(self.size);
        assert_eq!(
            self.size, input.size,
            "must run a circuit on inputs of its size"
        );
        Vector {
            size: input.size,
            vector: self.matrix * input.vector,
        }
    }
}

impl Mul<QCircuit> for QCircuit {
    type Output = QCircuit;

    /// Shorthand for `tensor_product`
    fn mul(self, other: QCircuit) -> QCircuit {
        self.tensor_product(other)
    }
}

impl Add<QCircuit> for QCircuit {
    type Output = QCircuit;

    /// Shorthand for `compose`
    fn add(self, other: QCircuit) -> QCircuit {
        self.compose(other)
    }
}
