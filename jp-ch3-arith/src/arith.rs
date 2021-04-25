use once_cell::sync::Lazy;
use std::any::Any;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct Value(Arc<dyn Any>);

impl Value {
    pub fn new<T: Any + Clone>(value: T) -> Value {
        Value(Arc::new(value))
    }

    pub fn cast<T: Any + Clone>(&self) -> Option<T> {
        self.0.downcast_ref::<T>().map(|v| v.clone())
    }
}

pub type DisplayProc = fn(Value) -> Option<String>;
pub type OpProc = fn(Value, Value) -> Option<Value>;

pub fn register_display(display: DisplayProc) {
    let mut arith = ARITH.write().unwrap();
    arith.displayers.push(display);
}

pub fn register_add(adder: OpProc) {
    let mut arith = ARITH.write().unwrap();
    arith.adders.push(adder);
}

pub fn register_sub(suber: OpProc) {
    let mut arith = ARITH.write().unwrap();
    arith.subers.push(suber);
}

pub fn register_mul(muler: OpProc) {
    let mut arith = ARITH.write().unwrap();
    arith.mulers.push(muler);
}

pub fn register_div(diver: OpProc) {
    let mut arith = ARITH.write().unwrap();
    arith.divers.push(diver);
}

static ARITH: Lazy<RwLock<Arithmetic>> = Lazy::new(|| RwLock::new(Arithmetic::new()));

fn display(arith: &Arithmetic, value: &Value) -> String {
    for displayer in &arith.displayers {
        if let Some(string) = displayer(value.clone()) {
            return string;
        }
    }
    panic!("No display handler matched");
}

fn apply_op(handlers: &[OpProc], left: &Value, right: &Value) -> Value {
    for op in handlers {
        if let Some(answer) = op(left.clone(), right.clone()) {
            return answer;
        }
    }
    panic!("No op handler matched");
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let arith = ARITH.read().unwrap();
        write!(f, "{}", display(&arith, self))
    }
}

impl Add<Value> for Value {
    type Output = Value;
    fn add(self, other: Value) -> Value {
        let arith = ARITH.read().unwrap();
        apply_op(&arith.adders, &self, &other)
    }
}

impl Sub<Value> for Value {
    type Output = Value;
    fn sub(self, other: Value) -> Value {
        let arith = ARITH.read().unwrap();
        apply_op(&arith.subers, &self, &other)
    }
}

impl Mul<Value> for Value {
    type Output = Value;
    fn mul(self, other: Value) -> Value {
        let arith = ARITH.read().unwrap();
        apply_op(&arith.mulers, &self, &other)
    }
}

impl Div<Value> for Value {
    type Output = Value;
    fn div(self, other: Value) -> Value {
        let arith = ARITH.read().unwrap();
        apply_op(&arith.divers, &self, &other)
    }
}

struct Arithmetic {
    displayers: Vec<DisplayProc>,
    adders: Vec<OpProc>,
    subers: Vec<OpProc>,
    mulers: Vec<OpProc>,
    divers: Vec<OpProc>,
}

impl Arithmetic {
    fn new() -> Arithmetic {
        Arithmetic {
            displayers: Vec::new(),
            adders: Vec::new(),
            subers: Vec::new(),
            mulers: Vec::new(),
            divers: Vec::new(),
        }
    }
}
