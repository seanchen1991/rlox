use super::instruction::{Value, Instruction};

const VECTOR_SIZE: usize = 8;

pub struct Chunk {
    pub code: Vec<Instruction>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { 
            code: Vec::with_capacity(VECTOR_SIZE),
            constants: Vec::with_capacity(VECTOR_SIZE),
        }
    }

    pub fn write_instruction(&mut self, inst: Instruction) {
        self.code.push(inst);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==\n", name);

        for (index, instruction) in self.code.iter().enumerate() {
            println!("{:04} {}", index, instruction);
        }
    }
}