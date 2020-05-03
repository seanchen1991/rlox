use super::chunk::Chunk;
use super::value::Value;
use super::instruction::OpCode;

const STACK_SIZE: usize = 256;

pub enum InterpretError {
    CompileError,
    RuntimeError,
}

type InterpretResult = Result<(), InterpretError>;

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
        use OpCode::*;

        loop {
            self.ip += 1;

            match self.get_current_instruction() {
                Constant(val) => self.stack.push(val.clone()),
                Add | Subtract | Multiply | Divide => {
                    if let Some(result) = self.binary_instruction() {
                        self.stack.push(result);
                    } else {
                        self.print_error("Invalid binary operation");
                        return Err(InterpretError::RuntimeError);
                    }
                },
                Negate => {
                    if let Some(result) = self.unary_instruction() {
                        self.stack.push(result);
                    } else {
                        self.print_error("Invalid unary operation");
                        return Err(InterpretError::RuntimeError);
                    }
                },
                Return => break,
            }
        }

        Ok(())
    }

    fn get_current_instruction(&self) -> OpCode {
        self.chunk.code[self.ip - 1].op.clone()
    }

    fn print_error(&self, message: &str) {
        println!("[Line: {}] Runtime error: {}", self.chunk.code[self.ip - 1].line, message);
    }

    fn unary_instruction(&mut self) -> Option<Value> {
        use OpCode::*;

        match self.get_current_instruction() {
            Negate => {
                match self.stack.pop() {
                    Some(val) => -val,
                    None => None,
                }
            },
            _ => {
                // TODO: Raise a runtime error instead of panicking
                panic!("Unary instruction was matched with an incorrect opcode!")
            }
        }
    }

    fn binary_instruction(&mut self) -> Option<Value> {
        use OpCode::*;

        // TODO: Raise a runtime error instead of panicking
        let b = self.stack.pop().expect("Trying to pop from empty stack!");
        let a = self.stack.pop().expect("Trying to pop from empty stack!");

        match self.get_current_instruction() {
            Add => a + b,
            Subtract => a - b,
            Multiply => a * b,
            Divide => a / b,
            _ => {
                // TODO: Raise a runtime error instead of panicking
                panic!("Binary instruction with an incorrect opcode!")
            }
        }
    }
}