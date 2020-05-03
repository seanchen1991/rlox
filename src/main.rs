mod vm;
mod chunk;
mod instruction;

use vm::VM;
use chunk::Chunk;
use instruction::{Value, OpCode, Instruction};

fn main() {
    let mut vm = VM::new();
    let mut chunk = Chunk::new();
    chunk.write_instruction(Instruction { op: OpCode::Return, line: 0 });
    chunk.write_instruction(Instruction { op: OpCode::Constant(Value::Number(64.0)), line: 1 });
    chunk.disassemble("test chunk");
}
