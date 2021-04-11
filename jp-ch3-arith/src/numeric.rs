use super::arith::{Arithmetic, Expr, Value};

pub fn num(n: f32) -> Expr {
    Expr::new_value(n)
}

fn numeric_display(x: Value) -> Option<String> {
    let x: f32 = x.cast()?;
    Some(format!("{}", x))
}

fn numeric_add(x: Value, y: Value) -> Option<Value> {
    let x: f32 = x.cast()?;
    let y: f32 = y.cast()?;
    Some(Value::new(x + y))
}

fn numeric_sub(x: Value, y: Value) -> Option<Value> {
    let x: f32 = x.cast()?;
    let y: f32 = y.cast()?;
    Some(Value::new(x - y))
}

fn numeric_mul(x: Value, y: Value) -> Option<Value> {
    let x: f32 = x.cast()?;
    let y: f32 = y.cast()?;
    Some(Value::new(x * y))
}

fn numeric_div(x: Value, y: Value) -> Option<Value> {
    let x: f32 = x.cast()?;
    let y: f32 = y.cast()?;
    Some(Value::new(x / y))
}

pub fn register_numeric_arithmetic(arithmetic: &mut Arithmetic) {
    arithmetic.register_display(fn display(x: Value) -> Option<String> {
      let x: f32 = x.cast()?;
      Some(format!("{}", x))
    })
    arithmetic.register_add(numeric_add);
    arithmetic.register_sub(numeric_sub);
    arithmetic.register_mul(numeric_mul);
    arithmetic.register_div(numeric_div);
}
