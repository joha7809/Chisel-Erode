use crate::isa::{InstrFormat, Instruction, Opcode, Operand, REGISTER_LIMIT};

#[derive(Debug)]
pub enum EncodeError {
    InvalidOperandCount {
        opcode: Opcode,
        expected: usize,
        found: usize,
    },
    RegisterOutOfRange(u8),
    ImmediateOutOfRange {
        bits: u8,
        value: usize,
    },
}

impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EncodeError::InvalidOperandCount {
                opcode,
                expected,
                found,
            } => write!(
                f,
                "Encode error: {} expects {} operands, found {}",
                opcode.to_string(),
                expected,
                found
            ),
            EncodeError::RegisterOutOfRange(r) => {
                write!(
                    f,
                    "Encode error: register R{} is out of range (1..={})",
                    r, REGISTER_LIMIT
                )
            }
            EncodeError::ImmediateOutOfRange { bits, value } => write!(
                f,
                "Encode error: immediate value {} does not fit in {} bits",
                value, bits
            ),
        }
    }
}

impl std::error::Error for EncodeError {}

// Bit field helpers (bit 31 is MSB)
const OPCODE_HI: u8 = 31;
const OPCODE_LO: u8 = 27; // 5 bits

fn set_bits(word: &mut u32, hi: u8, lo: u8, value: u32) {
    debug_assert!(hi >= lo);
    let width = (hi - lo + 1) as u32;
    let mask: u32 = if width == 32 {
        u32::MAX
    } else {
        (1u32 << width) - 1
    };
    let shift = lo as u32;
    *word |= (value & mask) << shift;
}

fn fits_in_bits(value: usize, bits: u8) -> bool {
    if bits >= 31 {
        // 31 or 32 bits (unsigned check)
        return true;
    }
    value <= ((1usize << bits) - 1)
}

fn encode_r3(op: Opcode, regs: &[Operand]) -> Result<u32, EncodeError> {
    if regs.len() != 3 {
        return Err(EncodeError::InvalidOperandCount {
            opcode: op,
            expected: 3,
            found: regs.len(),
        });
    }
    let (r1, r2, r3) = match (&regs[0], &regs[1], &regs[2]) {
        (Operand::Register(a), Operand::Register(b), Operand::Register(c)) => (*a, *b, *c),
        _ => {
            return Err(EncodeError::InvalidOperandCount {
                opcode: op,
                expected: 3,
                found: regs.len(),
            });
        }
    };

    // Validate register range (1..=REGISTER_LIMIT). Machine field is 5 bits (0..31), we map directly.
    for r in [r1, r2, r3] {
        if (r as usize) > REGISTER_LIMIT {
            return Err(EncodeError::RegisterOutOfRange(r));
        }
    }

    let mut word: u32 = 0;
    // opcode [31:27]
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    // r1 [26:22], r2 [21:17], r3 [16:12]
    set_bits(&mut word, 26, 22, r1 as u32);
    set_bits(&mut word, 21, 17, r2 as u32);
    set_bits(&mut word, 16, 12, r3 as u32);
    Ok(word)
}

fn encode_r2(op: Opcode, regs: &[Operand]) -> Result<u32, EncodeError> {
    if regs.len() != 2 {
        return Err(EncodeError::InvalidOperandCount {
            opcode: op,
            expected: 2,
            found: regs.len(),
        });
    }
    let (r1, r2) = match (&regs[0], &regs[1]) {
        (Operand::Register(a), Operand::Register(b)) => (*a, *b),
        _ => {
            return Err(EncodeError::InvalidOperandCount {
                opcode: op,
                expected: 2,
                found: regs.len(),
            });
        }
    };
    for r in [r1, r2] {
        if (r as usize) > REGISTER_LIMIT {
            return Err(EncodeError::RegisterOutOfRange(r));
        }
    }

    let mut word: u32 = 0;
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    set_bits(&mut word, 26, 22, r1 as u32);
    set_bits(&mut word, 21, 17, r2 as u32);
    Ok(word)
}

fn encode_ri(op: Opcode, ops: &[Operand]) -> Result<u32, EncodeError> {
    if ops.len() != 2 {
        return Err(EncodeError::InvalidOperandCount {
            opcode: op,
            expected: 2,
            found: ops.len(),
        });
    }
    let (r1, imm) = match (&ops[0], &ops[1]) {
        (Operand::Register(a), Operand::Immediate(b)) => (*a, *b),
        _ => {
            return Err(EncodeError::InvalidOperandCount {
                opcode: op,
                expected: 2,
                found: ops.len(),
            });
        }
    };
    if (r1 as usize) > REGISTER_LIMIT {
        return Err(EncodeError::RegisterOutOfRange(r1));
    }
    let imm_bits: u8 = 22;
    if !fits_in_bits(imm, imm_bits) {
        return Err(EncodeError::ImmediateOutOfRange {
            bits: imm_bits,
            value: imm,
        });
    }

    let mut word: u32 = 0;
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    set_bits(&mut word, 26, 22, r1 as u32);
    set_bits(&mut word, 21, 0, imm as u32);
    Ok(word)
}

fn encode_rri(op: Opcode, ops: &[Operand]) -> Result<u32, EncodeError> {
    if ops.len() != 3 {
        return Err(EncodeError::InvalidOperandCount {
            opcode: op,
            expected: 3,
            found: ops.len(),
        });
    }
    let (r1, r2, imm) = match (&ops[0], &ops[1], &ops[2]) {
        (Operand::Register(a), Operand::Register(b), Operand::Immediate(c)) => (*a, *b, *c),
        _ => {
            return Err(EncodeError::InvalidOperandCount {
                opcode: op,
                expected: 3,
                found: ops.len(),
            });
        }
    };
    for r in [r1, r2] {
        if (r as usize) > REGISTER_LIMIT {
            return Err(EncodeError::RegisterOutOfRange(r));
        }
    }
    let imm_bits: u8 = 17;
    if !fits_in_bits(imm, imm_bits) {
        return Err(EncodeError::ImmediateOutOfRange {
            bits: imm_bits,
            value: imm,
        });
    }

    let mut word: u32 = 0;
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    set_bits(&mut word, 26, 22, r1 as u32);
    set_bits(&mut word, 21, 17, r2 as u32);
    set_bits(&mut word, 16, 0, imm as u32);
    Ok(word)
}

fn encode_rii(op: Opcode, ops: &[Operand]) -> Result<u32, EncodeError> {
    if ops.len() != 3 {
        return Err(EncodeError::InvalidOperandCount {
            opcode: op,
            expected: 3,
            found: ops.len(),
        });
    }
    let (r1, imm1, imm2) = match (&ops[0], &ops[1], &ops[2]) {
        (Operand::Register(a), Operand::Immediate(b), Operand::Immediate(c)) => (*a, *b, *c),
        _ => {
            return Err(EncodeError::InvalidOperandCount {
                opcode: op,
                expected: 3,
                found: ops.len(),
            });
        }
    };
    if (r1 as usize) > REGISTER_LIMIT {
        return Err(EncodeError::RegisterOutOfRange(r1));
    }
    let bits = 11;
    if !fits_in_bits(imm1, bits) {
        return Err(EncodeError::ImmediateOutOfRange { bits, value: imm1 });
    }
    if !fits_in_bits(imm2, bits) {
        return Err(EncodeError::ImmediateOutOfRange { bits, value: imm2 });
    }

    let mut word: u32 = 0;
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    set_bits(&mut word, 26, 22, r1 as u32);
    set_bits(&mut word, 21, 11, imm1 as u32);
    set_bits(&mut word, 10, 0, imm2 as u32);
    Ok(word)
}

fn encode_i(op: Opcode, ops: &[Operand]) -> Result<u32, EncodeError> {
    // For JR: one immediate; for NOP/END we'll use zero immediate if none provided
    let imm: usize = match ops.first() {
        Some(Operand::Immediate(v)) => *v,
        None => 0,
        _ => {
            return Err(EncodeError::InvalidOperandCount {
                opcode: op,
                expected: 1,
                found: ops.len(),
            });
        }
    };
    let bits = 27;
    if !fits_in_bits(imm, bits) {
        return Err(EncodeError::ImmediateOutOfRange { bits, value: imm });
    }
    let mut word: u32 = 0;
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    set_bits(&mut word, 26, 0, imm as u32);
    Ok(word)
}

fn encode_noop_like(op: Opcode) -> u32 {
    // Encode only opcode in the top 5 bits; everything else zero
    let mut word: u32 = 0;
    set_bits(&mut word, OPCODE_HI, OPCODE_LO, op.code() as u32);
    word
}

pub fn encode_instruction(instr: &Instruction) -> Result<u32, EncodeError> {
    let opcode = instr.opcode.as_ref();
    let operands: Vec<Operand> = instr.operands.iter().map(|i| i.as_ref().clone()).collect();

    match opcode.instruction_format() {
        InstrFormat::R3 => encode_r3(*opcode, &operands),
        InstrFormat::R2 => encode_r2(*opcode, &operands),
        InstrFormat::RI => encode_ri(*opcode, &operands),
        InstrFormat::RRI => encode_rri(*opcode, &operands),
        InstrFormat::RII => encode_rii(*opcode, &operands),
        InstrFormat::I => encode_i(*opcode, &operands),
        InstrFormat::NoOP => Ok(encode_noop_like(*instr.opcode.as_ref())),
    }
}

pub fn encode_program(program: &[Instruction]) -> Result<Vec<u32>, EncodeError> {
    program.iter().map(encode_instruction).collect()
}
