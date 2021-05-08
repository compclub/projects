use std::fmt;
use std::ops::{Add, Sub};

pub type Complex = nalgebra::Complex<f64>;

/// A vector of length (2^n) of complex numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    pub(crate) size: usize,
    pub(crate) vector: nalgebra::DVector<Complex>,
}

/// Parse a complex number from a string. You can also use `Complex::new(re, im)`.
pub fn num(s: &str) -> Complex {
    use std::str::FromStr;
    Complex::from_str(s).unwrap()
}

impl Vector {
    pub fn zero(size: usize) -> Vector {
        let mut vector = Vec::new();
        vector.push(Complex::new(1.0, 0.0));
        for _ in 1..1 << size {
            vector.push(Complex::new(0.0, 0.0));
        }
        Vector {
            size,
            vector: nalgebra::DVector::from(vector),
        }
    }

    pub fn new(size: usize, elems: &[Complex]) -> Vector {
        assert_eq!(1 << size, elems.len(), "wrong number of elems in vector");
        Vector {
            size,
            vector: nalgebra::DVector::from(elems.to_owned()),
        }
    }

    pub fn tensor(self, other: Vector) -> Vector {
        Vector {
            size: self.size + other.size,
            vector: self.vector.kronecker(&other.vector),
        }
    }

    pub fn measure(&self) -> String {
        use std::fmt::Write;

        let mut output = String::new();
        let mut distribution = Vec::new();
        for (i, amplitude) in self.vector.iter().enumerate() {
            let bitstring = index_to_bitstring(i, self.size);
            let probability = amplitude.norm_sqr();
            distribution.push((probability, bitstring));
        }
        distribution.sort_by(|(p1, _), (p2, _)| p2.partial_cmp(p1).unwrap());
        for (p, bitstring) in distribution {
            write!(&mut output, "{:.3}: {}\n", p, bitstring).unwrap();
        }
        output
    }

    pub fn norm(&self) -> f64 {
        self.vector.norm()
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, amplitude) in self.vector.iter().enumerate() {
            let bitstring = index_to_bitstring(i, self.size);
            write!(f, "{}: {:.3}\n", bitstring, amplitude)?;
        }
        Ok(())
    }
}

fn index_to_bitstring(mut i: usize, size: usize) -> String {
    let mut bits = Vec::new();
    for _ in 0..size {
        let bit = if i % 2 == 0 { '0' } else { '1' };
        bits.push(bit);
        i = i / 2;
    }
    let mut bitstring = String::new();
    for bit in bits.into_iter().rev() {
        bitstring.push(bit);
    }
    bitstring
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        assert_eq!(self.size, other.size);
        Vector {
            size: self.size,
            vector: self.vector + other.vector,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        assert_eq!(self.size, other.size);
        Vector {
            size: self.size,
            vector: self.vector - other.vector,
        }
    }
}

#[test]
fn test_complex_numbers() {
    assert_eq!(format!("{}", num("1")), "1+0i");
    assert_eq!(format!("{}", num("i")), "0+1i");
    assert_eq!(num("2+3i") + num("-1.5+i"), num("0.5+4i"));
    assert_eq!(num("2+2i") - num("2+3i"), num("0-1i"));
    assert_eq!(num("1-i") * num("1+i"), num("2+0i"));
    assert_eq!(num("1-i") / num("1+i"), num("0-1i"));
    assert_eq!(num("1") / num("i"), num("0-1i"));
    assert_eq!(num("1") / num("-i"), num("0+i"));
}

#[test]
fn test_index_to_bitstring() {
    let bitstrings = (0..4).map(|i| index_to_bitstring(i, 2)).collect::<Vec<_>>();
    assert_eq!(bitstrings, &["00", "01", "10", "11"]);
}

#[test]
fn test_measure_vector() {
    let zero = Complex::new(0.0, 0.0);
    let one = Complex::new(1.0, 0.0);
    let i = Complex::new(0.0, 1.0);
    let v = Vector::new(2, &[one, zero, 0.4 * i, -0.5 * one]);
    assert_eq!(v.measure(), "1.000: 00\n0.250: 11\n0.160: 10\n0.000: 01\n");
}
