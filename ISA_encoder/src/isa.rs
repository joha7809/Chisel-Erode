use crate::errors::ParseError;

pub const REGISTER_LIMIT: usize = 32; //0..31
pub const REGISTER_BIT: usize = 32;
type OperandValidator = fn(&[Operand]) -> Result<(), ParseError>;

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
    Register(u8),     // 0..31
    Immediate(usize), // -2^31..2^31-1
}

#[derive(Debug, Clone)]
pub struct Instruction {
    // Opcode followed by either registers or immediate values
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum InstrFormat {
    R2,
    R3,  // opcode + reg + reg + reg
    RI,  // opcode + reg + imm
    RRI, // opcode + reg + reg + imm
    RII, // opcode + reg + imm + imm
    I,   // opcode + imm
    NoOP,
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

    pub fn to_string(self) -> &'static str {
        match self {
            Opcode::ADD => "ADD",
            Opcode::SUB => "SUB",
            Opcode::MULT => "MULT",
            Opcode::ADDI => "ADDI",
            Opcode::SUBI => "SUBI",
            Opcode::OR => "OR",
            Opcode::AND => "AND",
            Opcode::NOT => "NOT",
            Opcode::LI => "LI",
            Opcode::LD => "LD",
            Opcode::SD => "SD",
            Opcode::JR => "JR",
            Opcode::JGT => "JGT",
            Opcode::JEQ => "JEQ",
            Opcode::JLT => "JLT",
            Opcode::JETV => "JETV",
            Opcode::NOP => "NOP",
            Opcode::END => "END",
        }
    }

    pub fn code(self) -> u8 {
        match self {
            // ALU
            Opcode::ADD => 0b00001,
            Opcode::SUB => 0b00010,
            Opcode::MULT => 0b00011,
            Opcode::ADDI => 0b00100,
            Opcode::SUBI => 0b00101,
            Opcode::OR => 0b00110,
            Opcode::NOT => 0b00111,
            Opcode::AND => 0b10000, // updated unique code

            // DATA TRANSFER
            Opcode::LI => 0b01000,
            Opcode::LD => 0b01001,
            Opcode::SD => 0b01010,

            // CONTROL
            Opcode::JR => 0b01011,
            Opcode::JEQ => 0b01100,
            Opcode::JLT => 0b01101,
            Opcode::JGT => 0b01110,
            Opcode::JETV => 0b01111,
            Opcode::NOP => 0b00000,
            Opcode::END => 0b11111,
        }
    }

    pub fn instruction_format(self) -> InstrFormat {
        match self {
            // ALU instructions
            Opcode::ADD | Opcode::SUB | Opcode::MULT | Opcode::OR | Opcode::AND => InstrFormat::R3, // opcode + reg + reg + reg

            Opcode::NOT => InstrFormat::R2, // opcode + reg + reg

            Opcode::LI => InstrFormat::RI, // opcode + reg + imm
            Opcode::ADDI | Opcode::SUBI => InstrFormat::RRI,

            // Data transfer
            Opcode::LD | Opcode::SD => InstrFormat::R2, // opcode + reg + reg + imm (assuming memory offset as imm)

            // Control flow
            Opcode::JR => InstrFormat::I, // opcode + imm (jump target)
            Opcode::JLT => InstrFormat::RII,
            Opcode::JEQ | Opcode::JGT => InstrFormat::RRI, // opcode + reg + reg + imm (jump target)
            Opcode::JETV => InstrFormat::RII,              // opcode + reg + imm + imm
            Opcode::NOP | Opcode::END => InstrFormat::NoOP, // opcode + imm (NOP = 0, END = 27-bit placeholder)
        }
    }
    pub fn operand_validator(self) -> OperandValidator {
        match self.instruction_format() {
            crate::isa::InstrFormat::R3 => validate_r3,
            crate::isa::InstrFormat::R2 => validate_r2,
            crate::isa::InstrFormat::RI => validate_ri,
            crate::isa::InstrFormat::RRI => validate_rri,
            crate::isa::InstrFormat::RII => validate_rii,
            crate::isa::InstrFormat::I => validate_i,
            crate::isa::InstrFormat::NoOP => |_ops: &[Operand]| Ok(()),
        }
    }
}

fn validate_r3(ops: &[Operand]) -> Result<(), ParseError> {
    if ops.len() != 3 {
        return Err(ParseError::OperandCountMismatch {
            expected: 3,
            found: ops.len(),
        });
    }
    match (&ops[0], &ops[1], &ops[2]) {
        (Operand::Register(_), Operand::Register(_), Operand::Register(_)) => Ok(()),
        _ => Err(ParseError::OperandTypeMismatch),
    }
}

fn validate_r2(ops: &[Operand]) -> Result<(), ParseError> {
    if ops.len() != 2 {
        return Err(ParseError::OperandCountMismatch {
            expected: 2,
            found: ops.len(),
        });
    }
    match (&ops[0], &ops[1]) {
        (Operand::Register(_), Operand::Register(_)) => Ok(()),
        _ => Err(ParseError::OperandTypeMismatch),
    }
}

fn validate_ri(ops: &[Operand]) -> Result<(), ParseError> {
    if ops.len() != 2 {
        return Err(ParseError::OperandCountMismatch {
            expected: 2,
            found: ops.len(),
        });
    }
    match (&ops[0], &ops[1]) {
        (Operand::Register(_), Operand::Immediate(_)) => Ok(()),
        _ => Err(ParseError::OperandTypeMismatch),
    }
}

fn validate_rri(ops: &[Operand]) -> Result<(), ParseError> {
    if ops.len() != 3 {
        return Err(ParseError::OperandCountMismatch {
            expected: 3,
            found: ops.len(),
        });
    }
    match (&ops[0], &ops[1], &ops[2]) {
        (Operand::Register(_), Operand::Register(_), Operand::Immediate(_)) => Ok(()),
        _ => Err(ParseError::OperandTypeMismatch),
    }
}

fn validate_rii(ops: &[Operand]) -> Result<(), ParseError> {
    if ops.len() != 3 {
        return Err(ParseError::OperandCountMismatch {
            expected: 3,
            found: ops.len(),
        });
    }
    match (&ops[0], &ops[1], &ops[2]) {
        (Operand::Register(_), Operand::Immediate(_), Operand::Immediate(_)) => Ok(()),
        _ => Err(ParseError::OperandTypeMismatch),
    }
}

fn validate_i(ops: &[Operand]) -> Result<(), ParseError> {
    if ops.len() != 1 {
        return Err(ParseError::OperandCountMismatch {
            expected: 1,
            found: ops.len(),
        });
    }
    match &ops[0] {
        Operand::Immediate(_) => Ok(()),
        _ => Err(ParseError::OperandTypeMismatch),
    }
}
