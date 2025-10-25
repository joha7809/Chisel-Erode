// Core ISA definitions shared between assembler and VM
// This crate contains only the essential types and constants needed by both

use std::str::FromStr;

pub const REGISTER_LIMIT: usize = 32; // 0..31
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
    JLTV,
    JGT,
    JETV,
    NOP,
    END,
}

impl Opcode {
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
            Opcode::JLTV => "JLTV",
            Opcode::JETV => "JETV",
            Opcode::NOP => "NOP",
            Opcode::END => "END",
        }
    }

    /// Returns the 5-bit binary encoding for this opcode
    pub const fn code(self) -> u8 {
        match self {
            // ALU
            Opcode::ADD => 0b00001,
            Opcode::SUB => 0b00010,
            Opcode::MULT => 0b00011,
            Opcode::ADDI => 0b00100,
            Opcode::SUBI => 0b00101,
            Opcode::OR => 0b00110,
            Opcode::NOT => 0b00111,
            Opcode::AND => 0b10000,

            // DATA TRANSFER
            Opcode::LI => 0b01000,
            Opcode::LD => 0b01001,
            Opcode::SD => 0b01010,

            // CONTROL
            Opcode::JR => 0b01011,
            Opcode::JEQ => 0b01100,
            Opcode::JLTV => 0b01101,
            Opcode::JGT => 0b01110,
            Opcode::JETV => 0b01111,
            Opcode::NOP => 0b00000,
            Opcode::END => 0b11111,
        }
    }

    /// Decode a 5-bit opcode value back to an Opcode enum
    pub fn from_code(code: u8) -> Option<Opcode> {
        match code {
            0b00001 => Some(Opcode::ADD),
            0b00010 => Some(Opcode::SUB),
            0b00011 => Some(Opcode::MULT),
            0b00100 => Some(Opcode::ADDI),
            0b00101 => Some(Opcode::SUBI),
            0b00110 => Some(Opcode::OR),
            0b00111 => Some(Opcode::NOT),
            0b10000 => Some(Opcode::AND),
            0b01000 => Some(Opcode::LI),
            0b01001 => Some(Opcode::LD),
            0b01010 => Some(Opcode::SD),
            0b01011 => Some(Opcode::JR),
            0b01100 => Some(Opcode::JEQ),
            0b01101 => Some(Opcode::JLTV),
            0b01110 => Some(Opcode::JGT),
            0b01111 => Some(Opcode::JETV),
            0b00000 => Some(Opcode::NOP),
            0b11111 => Some(Opcode::END),
            _ => None,
        }
    }

    pub fn instruction_format(self) -> InstrFormat {
        match self {
            // ALU instructions
            Opcode::ADD | Opcode::SUB | Opcode::MULT | Opcode::OR | Opcode::AND => InstrFormat::R3,
            Opcode::NOT => InstrFormat::R2,
            Opcode::LI => InstrFormat::RI,
            Opcode::ADDI | Opcode::SUBI => InstrFormat::RRI,
            // Data transfer
            Opcode::LD | Opcode::SD => InstrFormat::R2,
            // Control flow
            Opcode::JR => InstrFormat::I,
            Opcode::JLTV => InstrFormat::RII,
            Opcode::JEQ | Opcode::JGT => InstrFormat::RRI,
            Opcode::JETV => InstrFormat::RII,
            Opcode::NOP | Opcode::END => InstrFormat::NoOP,
        }
    }
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Opcode::*;
        Ok(match s {
            "ADD" => ADD,
            "SUB" => SUB,
            "MULT" => MULT,
            "ADDI" => ADDI,
            "SUBI" => SUBI,
            "OR" => OR,
            "AND" => AND,
            "NOT" => NOT,
            "LI" => LI,
            "LD" => LD,
            "SD" => SD,
            "JR" => JR,
            "JGT" => JGT,
            "JEQ" => JEQ,
            "JLTV" => JLTV,
            "JETV" => JETV,
            "NOP" => NOP,
            "END" => END,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(clippy::upper_case_acronyms)]
pub enum InstrFormat {
    R2,   // opcode + reg + reg
    R3,   // opcode + reg + reg + reg
    RI,   // opcode + reg + imm
    RRI,  // opcode + reg + reg + imm
    RII,  // opcode + reg + imm + imm
    I,    // opcode + imm
    NoOP, // opcode only
}

// Simple operand enum without parsing-specific types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    Register(u8),     // 0..31
    Immediate(usize), // Value that fits in the instruction format
}

// Bit manipulation helpers for encoding/decoding
// TODO: Move this to encoder and rewrite maybe
pub mod bits {
    pub fn get_bits(word: u32, hi: u8, lo: u8) -> u32 {
        debug_assert!(hi >= lo);
        debug_assert!(hi < 32);
        let width = (hi - lo + 1) as u32;
        let mask: u32 = if width == 32 {
            u32::MAX
        } else {
            (1u32 << width) - 1
        };
        (word >> lo) & mask
    }

    pub fn set_bits(word: &mut u32, hi: u8, lo: u8, value: u32) {
        debug_assert!(hi >= lo);
        debug_assert!(hi < 32);
        let width = (hi - lo + 1) as u32;
        let mask: u32 = if width == 32 {
            u32::MAX
        } else {
            (1u32 << width) - 1
        };
        let shift = lo as u32;
        *word |= (value & mask) << shift;
    }

    pub fn fits_in_bits(value: usize, bits: u8) -> bool {
        if bits >= 31 {
            return true;
        }
        value <= ((1usize << bits) - 1)
    }
}

/// Decoded instruction - useful for VM
/// Note this instruction implements Encode and Decode
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedInstruction {
    pub opcode: Opcode,
    pub operands: Vec<Operand>,
}

// Lets just return an option, since we wont really have failures here
pub trait ToResolvedInstr {
    fn to_resolved(&self) -> Option<ResolvedInstruction>;
}

impl ResolvedInstruction {
    // TODO: Move to encoder, and implement it as trait
    /// Decode a 32-bit instruction word into its components
    pub fn decode(word: u32) -> Option<Self> {
        use bits::get_bits;

        // Extract opcode from bits [31:27]
        let opcode_bits = get_bits(word, 31, 27) as u8;
        let opcode = Opcode::from_code(opcode_bits)?;

        let operands = match opcode.instruction_format() {
            InstrFormat::R3 => {
                let r1 = get_bits(word, 26, 22) as u8;
                let r2 = get_bits(word, 21, 17) as u8;
                let r3 = get_bits(word, 16, 12) as u8;
                vec![
                    Operand::Register(r1),
                    Operand::Register(r2),
                    Operand::Register(r3),
                ]
            }
            InstrFormat::R2 => {
                let r1 = get_bits(word, 26, 22) as u8;
                let r2 = get_bits(word, 21, 17) as u8;
                vec![Operand::Register(r1), Operand::Register(r2)]
            }
            InstrFormat::RI => {
                let r1 = get_bits(word, 26, 22) as u8;
                let imm = get_bits(word, 21, 0) as usize;
                vec![Operand::Register(r1), Operand::Immediate(imm)]
            }
            InstrFormat::RRI => {
                let r1 = get_bits(word, 26, 22) as u8;
                let r2 = get_bits(word, 21, 17) as u8;
                let imm = get_bits(word, 16, 0) as usize;
                vec![
                    Operand::Register(r1),
                    Operand::Register(r2),
                    Operand::Immediate(imm),
                ]
            }
            InstrFormat::RII => {
                let r1 = get_bits(word, 26, 22) as u8;
                let imm1 = get_bits(word, 21, 11) as usize;
                let imm2 = get_bits(word, 10, 0) as usize;
                vec![
                    Operand::Register(r1),
                    Operand::Immediate(imm1),
                    Operand::Immediate(imm2),
                ]
            }
            InstrFormat::I => {
                let imm = get_bits(word, 26, 0) as usize;
                vec![Operand::Immediate(imm)]
            }
            InstrFormat::NoOP => vec![],
        };

        Some(ResolvedInstruction { opcode, operands })
    }

    /// Encode this instruction back to a 32-bit word
    pub fn encode(&self) -> Option<u32> {
        use bits::set_bits;

        let mut word: u32 = 0;
        set_bits(&mut word, 31, 27, self.opcode.code() as u32);

        match self.opcode.instruction_format() {
            InstrFormat::R3 => {
                if self.operands.len() != 3 {
                    return None;
                }
                if let [
                    Operand::Register(r1),
                    Operand::Register(r2),
                    Operand::Register(r3),
                ] = self.operands[..]
                {
                    set_bits(&mut word, 26, 22, r1 as u32);
                    set_bits(&mut word, 21, 17, r2 as u32);
                    set_bits(&mut word, 16, 12, r3 as u32);
                } else {
                    return None;
                }
            }
            InstrFormat::R2 => {
                if self.operands.len() != 2 {
                    return None;
                }
                if let [Operand::Register(r1), Operand::Register(r2)] = self.operands[..] {
                    set_bits(&mut word, 26, 22, r1 as u32);
                    set_bits(&mut word, 21, 17, r2 as u32);
                } else {
                    return None;
                }
            }
            InstrFormat::RI => {
                if self.operands.len() != 2 {
                    return None;
                }
                if let [Operand::Register(r1), Operand::Immediate(imm)] = self.operands[..] {
                    set_bits(&mut word, 26, 22, r1 as u32);
                    set_bits(&mut word, 21, 0, imm as u32);
                } else {
                    return None;
                }
            }
            InstrFormat::RRI => {
                if self.operands.len() != 3 {
                    return None;
                }
                if let [
                    Operand::Register(r1),
                    Operand::Register(r2),
                    Operand::Immediate(imm),
                ] = self.operands[..]
                {
                    set_bits(&mut word, 26, 22, r1 as u32);
                    set_bits(&mut word, 21, 17, r2 as u32);
                    set_bits(&mut word, 16, 0, imm as u32);
                } else {
                    return None;
                }
            }
            InstrFormat::RII => {
                if self.operands.len() != 3 {
                    return None;
                }
                if let [
                    Operand::Register(r1),
                    Operand::Immediate(imm1),
                    Operand::Immediate(imm2),
                ] = self.operands[..]
                {
                    set_bits(&mut word, 26, 22, r1 as u32);
                    set_bits(&mut word, 21, 11, imm1 as u32);
                    set_bits(&mut word, 10, 0, imm2 as u32);
                } else {
                    return None;
                }
            }
            InstrFormat::I => {
                if self.operands.len() != 1 {
                    return None;
                }
                if let [Operand::Immediate(imm)] = self.operands[..] {
                    set_bits(&mut word, 26, 0, imm as u32);
                } else {
                    return None;
                }
            }
            InstrFormat::NoOP => {
                // Already set opcode, nothing else needed
            }
        }

        Some(word)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opcode_roundtrip() {
        for opcode in [
            Opcode::ADD,
            Opcode::SUB,
            Opcode::MULT,
            Opcode::LI,
            Opcode::LD,
            Opcode::JR,
            Opcode::NOP,
            Opcode::END,
        ] {
            let code = opcode.code();
            assert_eq!(Opcode::from_code(code), Some(opcode));
        }
    }

    #[test]
    fn test_instruction_decode_encode() {
        // Test R3 format: ADD R1, R2, R3
        let instr = ResolvedInstruction {
            opcode: Opcode::ADD,
            operands: vec![
                Operand::Register(1),
                Operand::Register(2),
                Operand::Register(3),
            ],
        };
        let encoded = instr.encode().unwrap();
        let decoded = ResolvedInstruction::decode(encoded).unwrap();
        assert_eq!(instr, decoded);

        // Test RI format: LI R4, 100
        let instr = ResolvedInstruction {
            opcode: Opcode::LI,
            operands: vec![Operand::Register(4), Operand::Immediate(100)],
        };
        let encoded = instr.encode().unwrap();
        let decoded = ResolvedInstruction::decode(encoded).unwrap();
        assert_eq!(instr, decoded);
    }

    #[test]
    fn test_bit_manipulation() {
        use bits::*;

        let mut word = 0u32;
        set_bits(&mut word, 31, 27, 0b11111);
        assert_eq!(get_bits(word, 31, 27), 0b11111);

        set_bits(&mut word, 26, 22, 0b10101);
        assert_eq!(get_bits(word, 26, 22), 0b10101);
        assert_eq!(get_bits(word, 31, 27), 0b11111); // Previous bits unchanged
    }
}
