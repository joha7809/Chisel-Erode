use crate::{errors::ParseError, lexer::Span};

use isa_core::ToResolvedInstr;
// Re-export core ISA types from isa-core
pub use isa_core::{InstrFormat, Opcode, REGISTER_LIMIT};

// Assembler-specific operand type that includes label references
#[derive(Debug, Clone)]
pub enum Operand {
    Register(u8),     // 0..31
    Immediate(usize), // -2^31..2^31-1
    LabelRef(String), // Label reference (resolved during assembly)
}

// Spanned type for error reporting during parsing
#[derive(Debug, Clone)]
pub struct Spanned<T> {
    pub item: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn as_ref(&self) -> &T {
        &self.item
    }
    pub fn as_mut(&mut self) -> &mut T {
        &mut self.item
    }
    pub fn new(item: T, span: Span) -> Self {
        Self { item, span }
    }
}

// Assembler instruction with span information for error reporting
#[derive(Debug, Clone)]
pub struct UnresolvedInstruction {
    pub opcode: Spanned<Opcode>,
    pub operands: Vec<Spanned<Operand>>,
}

impl UnresolvedInstruction {
    pub fn get_total_span(&self) -> Span {
        if self.operands.is_empty() {
            return self.opcode.span;
        }
        let start_span = self.operands.first().unwrap();
        let end_span = self.operands.last().unwrap();
        Span {
            start: start_span.span.start,
            end: end_span.span.end,
            line: start_span.span.line,
        }
    }
}

impl ToResolvedInstr for UnresolvedInstruction {
    fn to_resolved(&self) -> Option<isa_core::ResolvedInstruction> {
        let mut resolved_ops = Vec::new();
        for op in &self.operands {
            match op.as_ref() {
                Operand::Register(r) => resolved_ops.push(isa_core::Operand::Register(*r)),
                Operand::Immediate(i) => resolved_ops.push(isa_core::Operand::Immediate(*i)),
                Operand::LabelRef(_) => {
                    // Cannot resolve label references here
                    // Should never happen if used correctly
                    return None;
                }
            }
        }
        Some(isa_core::ResolvedInstruction {
            opcode: *self.opcode.as_ref(),
            operands: resolved_ops,
        })
    }
}

enum OperandType {
    Reg,
    Imm,
}

fn validate_pattern(
    ops: &[Spanned<Operand>],
    instr_span: &Span,
    expected_count: usize,
    pattern: &[OperandType],
) -> Result<(), ParseError> {
    if ops.len() != expected_count {
        return Err(ParseError::OperandCountMismatch {
            expected: expected_count,
            found: ops.len(),
            span: *instr_span,
        });
    }

    for (op, expected) in ops.iter().zip(pattern) {
        match (op.as_ref(), expected) {
            (Operand::Register(_), OperandType::Reg)
            | (Operand::Immediate(_), OperandType::Imm) => continue,
            _ => return Err(ParseError::OperandTypeMismatch { span: op.span }),
        }
    }
    Ok(())
}

// Trait to add validation to InstrFormat in the assembler
pub trait InstrFormatValidator {
    fn validate(&self, ops: &[Spanned<Operand>], span: &Span) -> Result<(), ParseError>;
}

impl InstrFormatValidator for InstrFormat {
    fn validate(&self, ops: &[Spanned<Operand>], span: &Span) -> Result<(), ParseError> {
        use InstrFormat::*;
        match self {
            R3 => validate_pattern(
                ops,
                span,
                3,
                &[OperandType::Reg, OperandType::Reg, OperandType::Reg],
            ),
            R2 => validate_pattern(ops, span, 2, &[OperandType::Reg, OperandType::Reg]),
            RI => validate_pattern(ops, span, 2, &[OperandType::Reg, OperandType::Imm]),
            RRI => validate_pattern(
                ops,
                span,
                3,
                &[OperandType::Reg, OperandType::Reg, OperandType::Imm],
            ),
            RII => validate_pattern(
                ops,
                span,
                3,
                &[OperandType::Reg, OperandType::Imm, OperandType::Imm],
            ),
            I => validate_pattern(ops, span, 1, &[OperandType::Imm]),
            NoOP => Ok(()),
        }
    }
}
