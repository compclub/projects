use super::arith::{
    register_add, register_display, register_div, register_mul, register_sub, Value,
};

pub fn num(n: f32) -> Value {
    Value::new(n)
}

pub fn register_numeric_arithmetic() {
    register_display(numeric_display);
    register_add(numeric_add);
    register_sub(numeric_sub);
    register_mul(numeric_mul);
    register_div(numeric_div);
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
