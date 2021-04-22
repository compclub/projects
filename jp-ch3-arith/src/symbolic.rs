use super::arith::{
    register_add, register_display, register_div, register_mul, register_sub, Value,
};

pub fn sym(ch: char) -> Value {
    Value::new(format!("{}", ch))
}

fn to_symbols(x: Value, y: Value) -> Option<(String, String)> {
    let xsym = x.cast::<String>();
    let ysym = y.cast::<String>();
    // If at least one of the args is a symbol, do symbolic arith
    match (xsym, ysym) {
        (Some(xsym), Some(ysym)) => Some((xsym, ysym)),
        (Some(xsym), None) => Some((xsym, y.to_string())),
        (None, Some(ysym)) => Some((x.to_string(), ysym)),
        (None, None) => None,
    }
}

fn symbolic_display(x: Value) -> Option<String> {
    x.cast::<String>()
}

fn symbolic_add(x: Value, y: Value) -> Option<Value> {
    to_symbols(x, y).map(|(x, y)| Value::new(format!("({} + {})", x, y)))
}

fn symbolic_sub(x: Value, y: Value) -> Option<Value> {
    to_symbols(x, y).map(|(x, y)| Value::new(format!("({} - {})", x, y)))
}

fn symbolic_mul(x: Value, y: Value) -> Option<Value> {
    to_symbols(x, y).map(|(x, y)| Value::new(format!("({} * {})", x, y)))
}

fn symbolic_div(x: Value, y: Value) -> Option<Value> {
    to_symbols(x, y).map(|(x, y)| Value::new(format!("({} / {})", x, y)))
}

pub fn register_symbolic_arithmetic() {
    register_display(symbolic_display);
    register_add(symbolic_add);
    register_sub(symbolic_sub);
    register_mul(symbolic_mul);
    register_div(symbolic_div);
}
