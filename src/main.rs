use std::fmt;

#[repr(C)]
pub enum OpCode {
    Return,
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpCode::Return => write!(f, "RETURN"),
        }
    }
}

pub struct Chunk {
    code: Vec<OpCode>
}

impl Chunk {
    pub fn new() -> Self {
        Chunk { code: Vec::new() }
    }

    pub fn write(&mut self, op: OpCode) {
        self.code.push(op);
    }

    pub fn disassemble(&self, name: &str) {
        println!("== {} ==\n", name);

        let mut offset: usize = 0;

        for op in self.code.iter() {
            println!("{:04} {}", offset, op);
            offset += 1;
        }
    }
}

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::Return);
    chunk.disassemble("test chunk");
}
