use std::fmt;

use super::value::Value;

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use OpCode::*;

        match self {
            Return => write!(f, "OP_RETURN"),
            Constant(v) => write!(f, "OP_CONSTANT {}", v),
            Add => write!(f, "OP_ADD"),
            Subtract => write!(f, "OP_SUBTRACT"),
            Multiply => write!(f, "OP_MULTIPLY"),
            Divide => write!(f, "OP_DIVIDE"),
            Negate => write!(f, "OP_NEGATE"),
        }
    }
}

pub struct Instruction {
    pub op: OpCode,
    pub line: usize,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "L{:03} {}", self.line, self.op)
    }
}