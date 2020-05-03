use std::fmt;
use std::ops::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, EnumAsGetters, EnumIsA)]
pub enum Value {
    String(Rc<String>),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use Value::*;

        match self {
            String(s) => write!(f, "{}", s),
            Number(n) => write!(f, "{}", n),
            Bool(b) => write!(f, "{}", b),
            Nil => write!(f, "nil")
        }
    }
}

impl Add for Value {
    type Output = Option<Value>;

    fn add(self, other: Value) -> Option<Value> {
        if self.is_number() && other.is_number() {
            Some(Value::Number(self.as_number() + other.as_number()))
        } else {
            Some(Value::String(Rc::new(self.to_string() + &other.to_string())))
        }
    }
}

impl Sub for Value {
    type Output = Option<Value>;

    fn sub(self, other: Value) -> Option<Value> {
        if self.is_number() && other.is_number() {
            Some(Value::Number(self.as_number() - other.as_number()))
        } else {
            None
        }
    }
}

impl Mul for Value {
    type Output = Option<Value>;

    fn mul(self, other: Value) -> Option<Value> {
        if self.is_number() && other.is_number() {
            Some(Value::Number(self.as_number() * other.as_number()))
        } else {
            None
        }
    }
}

impl Div for Value {
    type Output = Option<Value>;

    fn div(self, other: Value) -> Option<Value> {
        if self.is_number() && other.is_number() {
            // TODO: Handle division by 0
            Some(Value::Number(self.as_number() / other.as_number()))
        } else {
            None
        }
    }
}

impl Neg for Value {
    type Output = Option<Value>;

    fn neg(self) -> Option<Value> {
        match self {
            Value::Number(val) => Some(Value::Number(-val)),
            _ => None, 
        }
    }
}