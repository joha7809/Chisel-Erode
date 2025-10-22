pub const REGISTER_LIMIT: usize = 32; //0..31
pub const REGISTER_BIT: usize = 32;

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

#[derive(Debug, Clone)]
pub enum UnresolvedOperand {
    Register(usize),
    Immediate(usize),
    LabelRef(String), // Temporary reference to a label
}

#[derive(Debug, Clone)]
pub struct UnresolvedInstruction {
    pub opcode: Opcode,
    pub operands: Vec<UnresolvedOperand>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrFormat {
    R2,
    R3,  // opcode + reg + reg + reg
    RI,  // opcode + reg + imm
    RRI, // opcode + reg + reg + imm
    RII, // opcode + reg + imm + imm
    I,   // opcode + imm
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

            Opcode::ADDI | Opcode::SUBI | Opcode::LI => InstrFormat::RI, // opcode + reg + imm

            // Data transfer
            Opcode::LD | Opcode::SD => InstrFormat::RRI, // opcode + reg + reg + imm (assuming memory offset as imm)

            // Control flow
            Opcode::JR => InstrFormat::I, // opcode + imm (jump target)
            Opcode::JEQ | Opcode::JLT | Opcode::JGT => InstrFormat::RRI, // opcode + reg + reg + imm (jump target)
            Opcode::JETV => InstrFormat::RII,                            // opcode + reg + imm + imm
            Opcode::NOP | Opcode::END => InstrFormat::I, // opcode + imm (NOP = 0, END = 27-bit placeholder)
        }
    }
}
