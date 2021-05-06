use once_cell::sync::Lazy;
use regex::Regex;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::str::FromStr;

static COMPLEX_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new("^([+-]?[0-9.]+)?([+-])?(([0-9.]*)i)?$").unwrap());

pub fn num(s: &str) -> Complex {
    let captures = match COMPLEX_REGEX.captures(s) {
        Some(capt) => capt,
        None => panic!("Failed to parse complex number {}", s),
    };
    let re = f32::from_str(captures.get(1).map_or("0", |m| m.as_str())).unwrap();
    let sign = match captures.get(2).map(|m| m.as_str()) {
        None => 1.0,
        Some("+") => 1.0,
        Some("-") => -1.0,
        _ => unreachable!(),
    };
    let im_str = captures.get(4).map_or("0", |m| m.as_str());
    let im = if im_str == "" {
        1.0
    } else {
        f32::from_str(im_str).unwrap()
    };
    Complex { re, im: sign * im }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f32,
    im: f32,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.re == 0.0 {
            if self.im == 0.0 {
                write!(f, "0")
            } else if self.im == 1.0 {
                write!(f, "i")
            } else if self.im == -1.0 {
                write!(f, "-i")
            } else if self.im > 0.0 {
                write!(f, "{}i", self.im)
            } else {
                write!(f, "-{}i", -self.im)
            }
        } else {
            if self.im == 0.0 {
                write!(f, "{}", self.re)
            } else if self.im == 1.0 {
                write!(f, "{}+i", self.re)
            } else if self.im == -1.0 {
                write!(f, "{}-i", self.re)
            } else if self.im > 0.0 {
                write!(f, "{}+{}i", self.re, self.im)
            } else {
                write!(f, "{}-{}i", self.re, -self.im)
            }
        }
    }
}

impl Complex {
    pub fn new(re: f32, im: f32) -> Complex {
        Complex { re, im }
    }
}

impl Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl Sub<Complex> for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl Mul<Complex> for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl Div<Complex> for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let norm = other.re * other.re + other.im * other.im;
        Complex {
            re: (self.re * other.re + self.im * other.im) / norm,
            im: (self.im * other.re - self.re * other.im) / norm,
        }
    }
}

#[test]
fn test_complex_numbers() {
    assert_eq!(format!("{}", num("1")), "1");
    assert_eq!(format!("{}", num("i")), "i");
    assert_eq!(num("2+3i") + num("-1.5+i"), num("0.5+4i"));
    assert_eq!(num("2+2i") - num("2+3i"), num("-i"));
    assert_eq!(num("1-i") * num("1+i"), num("2"));
    assert_eq!(num("1-i") / num("1+i"), num("-i"));
    assert_eq!(num("1") / num("i"), num("-i"));
    assert_eq!(num("1") / num("-i"), num("i"));
}
