use std::any::Any;
use std::collections::HashMap;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::Arc;

#[derive(Clone)]
pub struct Value(Arc<dyn Any>);

impl Value {
    pub fn new<T: Any + fmt::Display>(value: T) -> Value {
        Value(Arc::new(value))
    }

    pub fn cast<T: Any + Clone>(&self) -> Option<T> {
        self.0.downcast_ref::<T>().map(|v| v.clone())
    }
}

pub enum Expr {
    Value(Value),
    Op(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn new_value<T: Any + fmt::Display>(value: T) -> Expr {
        Expr::Value(Value::new(value))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Add<Expr> for Expr {
    type Output = Expr;
    fn add(self, other: Expr) -> Expr {
        Expr::Op(Op::Add, Box::new(self), Box::new(other))
    }
}

impl Sub<Expr> for Expr {
    type Output = Expr;
    fn sub(self, other: Expr) -> Expr {
        Expr::Op(Op::Sub, Box::new(self), Box::new(other))
    }
}

impl Mul<Expr> for Expr {
    type Output = Expr;
    fn mul(self, other: Expr) -> Expr {
        Expr::Op(Op::Mul, Box::new(self), Box::new(other))
    }
}

impl Div<Expr> for Expr {
    type Output = Expr;
    fn div(self, other: Expr) -> Expr {
        Expr::Op(Op::Div, Box::new(self), Box::new(other))
    }
}

pub type DisplayProc = fn(Value) -> Option<String>;
pub type OpProc = fn(Value, Value) -> Option<Value>;

pub struct Arithmetic {
    display_handlers: Vec<DisplayProc>,
    op_handlers: HashMap<Op, Vec<OpProc>>,
}

impl Arithmetic {
    pub fn new() -> Arithmetic {
        let mut op_handlers = HashMap::new();
        op_handlers.insert(Op::Add, vec![]);
        op_handlers.insert(Op::Sub, vec![]);
        op_handlers.insert(Op::Mul, vec![]);
        op_handlers.insert(Op::Div, vec![]);
        Arithmetic {
            op_handlers,
            display_handlers: vec![],
        }
    }

    pub fn register_display(&mut self, handler: DisplayProc) {
        self.display_handlers.push(handler);
    }

    pub fn register_add(&mut self, handler: OpProc) {
        self.register(Op::Add, handler)
    }

    pub fn register_sub(&mut self, handler: OpProc) {
        self.register(Op::Sub, handler)
    }

    pub fn register_mul(&mut self, handler: OpProc) {
        self.register(Op::Mul, handler)
    }

    pub fn register_div(&mut self, handler: OpProc) {
        self.register(Op::Div, handler)
    }

    fn register(&mut self, op: Op, handler: fn(Value, Value) -> Option<Value>) {
        self.op_handlers
            .entry(op)
            .or_insert_with(|| vec![])
            .push(handler);
    }

    pub fn display(&self, value: Value) -> Option<String> {
        for handler in &self.display_handlers {
            if let Some(string) = handler(value.clone()) {
                return Some(string);
            }
        }
        None
    }

    pub fn eval(&self, expr: &Expr) -> Option<Value> {
        match expr {
            Expr::Value(value) => Some(value.clone()),
            Expr::Op(op, x, y) => {
                let x = self.eval(x)?;
                let y = self.eval(y)?;
                let handlers = self.op_handlers.get(&op)?;
                for handler in handlers {
                    if let Some(result) = handler(x.clone(), y.clone()) {
                        return Some(result);
                    }
                }
                None
            }
        }
    }
}
