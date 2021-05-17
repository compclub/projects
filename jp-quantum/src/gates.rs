use crate::qcircuit::QCircuit;
use crate::vector::Complex;

pub const ZERO: Complex = Complex { re: 0.0, im: 0.0 };
pub const ONE: Complex = Complex { re: 1.0, im: 0.0 };
pub const I: Complex = Complex { re: 0.0, im: 1.0 };

/// An identity circuit for a single qbit
pub fn id() -> QCircuit {
    QCircuit::one_qbit_gate(&[ONE, ZERO, ZERO, ONE])
}

/// Hadamard gate
pub fn h() -> QCircuit {
    let plus = 0.7071 * ONE;
    QCircuit::one_qbit_gate(&[plus, plus, plus, -plus])
}

/// Pauli X gate
pub fn x() -> QCircuit {
    QCircuit::one_qbit_gate(&[ZERO, ONE, ONE, ZERO])
}

/// Pauli Y gate
pub fn y() -> QCircuit {
    QCircuit::one_qbit_gate(&[ZERO, -I, I, ZERO])
}

/// Pauli Z gate
pub fn z() -> QCircuit {
    QCircuit::one_qbit_gate(&[ONE, ZERO, ZERO, -ONE])
}

/// Controlled Z gate
pub fn cz() -> QCircuit {
    QCircuit::two_qbit_gate(&[
        ONE, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, -ONE,
    ])
}

/// Doubly controlled Z gate
pub fn ccz() -> QCircuit {
    QCircuit::three_qbit_gate(&[
        ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ZERO,
        ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO,
        ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO,
        ONE, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ZERO, ONE, ZERO, ZERO, ZERO, ZERO, ZERO,
        ZERO, ZERO, ZERO, -ONE,
    ])
}
