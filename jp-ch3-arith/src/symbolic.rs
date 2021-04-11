use super::arith::{Arithmetic, Expr, Value};

pub fn sym(ch: char) -> Expr {
    Expr::new_value(format!("{}", ch))
}

fn to_symbol(x: Value) -> Option<String> {
    if let Some(symbol) = x.cast::<String>() {
        Some(symbol.to_owned())
    } else if let Some(num) = x.cast::<f32>() {
        Some(format!("{}", num))
    } else {
        None
    }
}

fn symbolic_display(x: Value) -> Option<String> {
    to_symbol(x)
}

fn symbolic_add(x: Value, y: Value) -> Option<Value> {
    let x: String = to_symbol(x)?;
    let y: String = to_symbol(y)?;
    Some(Value::new(format!("({} + {})", x, y)))
}

fn symbolic_sub(x: Value, y: Value) -> Option<Value> {
    let x: String = to_symbol(x)?;
    let y: String = to_symbol(y)?;
    Some(Value::new(format!("({} - {})", x, y)))
}

fn symbolic_mul(x: Value, y: Value) -> Option<Value> {
    let x: String = to_symbol(x)?;
    let y: String = to_symbol(y)?;
    Some(Value::new(format!("({} * {})", x, y)))
}

fn symbolic_div(x: Value, y: Value) -> Option<Value> {
    let x: String = to_symbol(x)?;
    let y: String = to_symbol(y)?;
    Some(Value::new(format!("({} / {})", x, y)))
}

pub fn register_symbolic_arithmetic(arithmetic: &mut Arithmetic) {
    arithmetic.register_display(symbolic_display);
    arithmetic.register_add(symbolic_add);
    arithmetic.register_sub(symbolic_sub);
    arithmetic.register_mul(symbolic_mul);
    arithmetic.register_div(symbolic_div);
}
