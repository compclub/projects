mod arith;
mod numeric;
mod symbolic;

pub use arith::{Arithmetic, Value};
pub use numeric::{num, register_numeric_arithmetic};
pub use symbolic::{register_symbolic_arithmetic, sym};

#[cfg(test)]
mod test {
    use super::*;

    fn make_arithmetic() -> Arithmetic {
        let mut arith = Arithmetic::new();
        register_numeric_arithmetic(&mut arith);
        register_symbolic_arithmetic(&mut arith);
        arith
    }

    #[test]
    fn test_numeric_arith() {
        let arith = make_arithmetic();
        let expr = num(2.0) + num(3.0);
        let result = arith.eval(&expr).unwrap();
        assert_eq!(arith.display(result).unwrap(), "5");
    }

    #[test]
    fn test_symbolic_arith() {
        let arith = make_arithmetic();
        let expr = num(2.0) + sym('x');
        let result = arith.eval(&expr).unwrap();
        assert_eq!(arith.display(result).unwrap(), "(2 + x)");
    }
}
