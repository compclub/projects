mod arith;
mod numeric;
mod symbolic;
mod vector;

pub use arith::Value;
pub use numeric::{num, register_numeric_arithmetic};
pub use symbolic::{register_symbolic_arithmetic, sym};
pub use vector::{register_vector_arithmetic, vec};

#[test]
fn test_arithmetic() {
    register_numeric_arithmetic();
    register_symbolic_arithmetic();
    register_vector_arithmetic();

    let n = num(2.0) + num(3.5);
    assert_eq!(n.to_string(), "5.5");
    let n = num(2.0) + sym('x');
    assert_eq!(n.to_string(), "(2 + x)");
    let n = sym('x') + num(0.0);
    assert_eq!(n.to_string(), "(x + 0)");
    let n = (num(17.0) - num(7.0)) * (num(1.0) - sym('x'));
    assert_eq!(n.to_string(), "(10 * (1 - x))");

    let v1 = vec(vec![num(2.0), sym('x')]);
    let v2 = vec(vec![(num(1.0) + num(2.0)), num(0.0)]);
    let n = v1 * v2;
    assert_eq!(n.to_string(), "[6, (x * 0)]");
}
