pub const REGISTER_LIMIT: usize = 32; //0..31
pub const REGISTER_BIT: usize = 32;

#[derive(Clone, Copy)]
pub enum ArgType {
    Register,
    Immediate,
    Address,
    EndOfProgram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum Opcode {
    ADD,
    SUB,
    MULT,
    ADDI,
    SUBI,
    OR,
    AND,
    NOT,

    // DATA
    LI,
    LD,
    SD,

    // Control
    JR,
    JEQ,
    JLT,
    JGT,
    JETV,
    NOP,
    END,
}

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    Register(u8),   // 0..31
    Immediate(i32), // -2^31..2^31-1
    Address(u16),   // 0..65535
}

#[derive(Debug, Clone)]
pub struct Instruction {
    // Opcode followed by either registers or immediate values
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

impl Opcode {
    pub fn from_str(op: &str) -> Option<Opcode> {
        match op {
            "ADD" => Some(Opcode::ADD),
            "SUB" => Some(Opcode::SUB),
            "MULT" => Some(Opcode::MULT),
            "ADDI" => Some(Opcode::ADDI),
            "SUBI" => Some(Opcode::SUBI),
            "OR" => Some(Opcode::OR),
            "AND" => Some(Opcode::AND),
            "NOT" => Some(Opcode::NOT),
            "LI" => Some(Opcode::LI),
            "LD" => Some(Opcode::LD),
            "SD" => Some(Opcode::SD),
            "JR" => Some(Opcode::JR),
            "JGT" => Some(Opcode::JGT),
            "JEQ" => Some(Opcode::JEQ),
            "JLT" => Some(Opcode::JLT),
            "JETV" => Some(Opcode::JETV),
            "NOP" => Some(Opcode::NOP),
            "END" => Some(Opcode::END),
            _ => None,
        }
    }
    pub fn arg_types(&self) -> &'static [ArgType] {
        use ArgType::*;
        match self {
            Opcode::ADD | Opcode::SUB | Opcode::MULT | Opcode::OR | Opcode::AND => {
                &[Register, Register, Register]
            }
            Opcode::ADDI | Opcode::SUBI => &[Register, Register, Immediate],
            Opcode::NOT => &[Register, Register],
            Opcode::LI => &[Register, Immediate],
            Opcode::LD | Opcode::SD => &[Register, Register],
            Opcode::JR => &[Immediate],
            Opcode::JEQ | Opcode::JLT | Opcode::JGT => &[Immediate, Register, Register],
            Opcode::JETV => &[Immediate, Register, Immediate],
            Opcode::NOP => &[],
            Opcode::END => &[EndOfProgram],
        }
    }
}
