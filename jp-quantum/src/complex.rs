use std::str::FromStr;

pub type Complex = nalgebra::Complex<f32>;

pub fn num(s: &str) -> Complex {
    Complex::from_str(s).unwrap()
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
