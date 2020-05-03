use super::chunk::Chunk;
use super::instruction::{OpCode, Value};

const STACK_SIZE: usize = 256;

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

pub struct VM {
    chunk: Chunk,
    ip: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: Chunk::new(),
            ip: 0,
            stack: Vec::with_capacity(STACK_SIZE),
        }
    }

    pub fn with_chunk(chunk: Chunk) -> Self {
        VM {
            chunk,
            ip: 0,
            stack: Vec::with_capacity(STACK_SIZE)
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        self.run()
    }

    fn run(&mut self) -> InterpretResult {
        loop {
            self.ip += 1;

            match self.get_current_instruction() {
                OpCode::Constant(val) => self.stack.push(val.clone()),
                OpCode::Return => break,
            }
        }

        InterpretResult::Ok
    }

    fn get_current_instruction(&self) -> OpCode {
        self.chunk.code[self.ip - 1].op.clone()
    }
}