use super::arith::{register_add, register_display, register_mul, register_sub, Value};

pub fn vec(elems: Vec<Value>) -> Value {
    Value::new(elems)
}

pub fn register_vector_arithmetic() {
    register_display(vector_display);
    register_add(vector_add);
    register_sub(vector_sub);
    register_mul(vector_mul);
}

fn vector_display(x: Value) -> Option<String> {
    let x: Vec<Value> = x.cast()?;
    let strings: Vec<String> = x.into_iter().map(|x| x.to_string()).collect();
    Some(format!("[{}]", strings.join(", ")))
}

fn vector_op(x: Value, y: Value, op: fn((Value, Value)) -> Value) -> Option<Value> {
    let x: Vec<Value> = x.cast()?;
    let y: Vec<Value> = y.cast()?;
    assert_eq!(x.len(), y.len());
    let pairs = x.into_iter().zip(y.into_iter());
    Some(Value::new(pairs.map(op).collect::<Vec<_>>()))
}

fn vector_add(x: Value, y: Value) -> Option<Value> {
    vector_op(x, y, |(x, y)| x + y)
}

fn vector_sub(x: Value, y: Value) -> Option<Value> {
    vector_op(x, y, |(x, y)| x - y)
}

fn vector_mul(x: Value, y: Value) -> Option<Value> {
    // dot product
    let x: Vec<Value> = x.cast()?;
    let y: Vec<Value> = y.cast()?;
    assert_eq!(x.len(), y.len());
    let mut pairs = x.into_iter().zip(y.into_iter());
    let (first_x, first_y) = pairs.next().unwrap();
    let mut sum = first_x * first_y;
    for (next_x, next_y) in pairs {
        sum = sum + next_x * next_y;
    }
    Some(sum)
}
