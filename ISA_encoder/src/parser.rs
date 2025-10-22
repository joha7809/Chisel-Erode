use std::collections::HashMap;

use crate::{
    errors::ParseError,
    isa::{Instruction, Opcode, Operand},
    lexer::{Token, TokenKind},
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}
/// Returns a vec of instructions.
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn next(&mut self) -> Option<&Token> {
        let tok = self.tokens.get(self.position);
        if tok.is_some() {
            self.position += 1;
        }
        tok
    }

    fn set_pos(&mut self, pos: usize) {
        self.position = pos;
    }

    fn expect(&mut self, kind: TokenKind) -> Result<&Token, ParseError> {
        match self.next() {
            Some(tok) if tok.kind == kind => Ok(tok),
            Some(tok) => Err(ParseError::UnexpectedToken {
                expected: format!("{:?}", kind),
                found: format!("{:?}", tok.kind),
                position: tok.span.start,
            }),
            None => Err(ParseError::UnexpectedEndOfInput),
        }
    }

    // We need to handle label logic cleverly
    // Which datatype can we use for key of str and value of index of the label instructions
    pub fn parse_instructions(&mut self) -> Result<Vec<Instruction>, ParseError> {
        // Entry point for parsing logic
        // At each opcode, we parse_operands and collect the opcode
        let mut instructions: Vec<Instruction> = Vec::new();
        let label_map = self.generate_label_map()?;
        self.set_pos(0); // reset position for actual parsing
        while let Some(token) = self.peek() {
            // We need to skip untill opcode
            match &token.kind {
                TokenKind::Opcode(opcode) => {
                    let opcode = *opcode;
                    self.next(); // consume the opcode token
                    let operands = self.parse_operands_for(opcode, &label_map)?;
                    // DEBUG PRINT WHAT IS GOING on
                    println!("Parsed Opcode: {:?}, Operands: {:?}", opcode, operands);
                    opcode.operand_validator()(&operands)?; // validate immediately
                    instructions.push(Instruction { opcode, operands });
                }
                _ => {
                    self.next(); // consume non-opcode tokens
                }
            }
        }

        Ok(instructions)
    }

    /// Parse operands for a specific opcode. Resolves labels to immediate indices.
    fn parse_operands_for(
        &mut self,
        opcode: Opcode,
        label_map: &HashMap<String, usize>,
    ) -> Result<Vec<Operand>, ParseError> {
        //TODO: CLEANUP THIS CHAT CODE

        let expected = expected_operand_count(opcode);
        if expected == 0 {
            return Ok(Vec::new());
        }

        let mut operands: Vec<Operand> = Vec::with_capacity(expected);
        while operands.len() < expected {
            let token = match self.peek() {
                Some(t) => t.clone(),
                None => break,
            };

            match &token.kind {
                TokenKind::Register(reg_num) => {
                    operands.push(Operand::Register(*reg_num as u8));
                    self.next();
                }
                TokenKind::Immediate(imm_val) => {
                    operands.push(Operand::Immediate(*imm_val));
                    self.next();
                }
                TokenKind::LabelRef(label_name) => {
                    if let Some(idx) = label_map.get(label_name) {
                        operands.push(Operand::Immediate(*idx));
                        self.next();
                    } else {
                        return Err(ParseError::UndefinedLabel {
                            label: label_name.clone(),
                            position: token.span.start,
                        });
                    }
                }
                TokenKind::Comma
                | TokenKind::Comment(_)
                | TokenKind::Terminator
                | TokenKind::LabelDef(_) => {
                    // Skip separators/comments/label defs during operand parsing
                    self.next();
                }
                TokenKind::Opcode(_) => {
                    // Next instruction begins; insufficient operands
                    break;
                }
            }
        }

        Ok(operands)
    }

    /// First pass, simply scans the token stream for label def, and maps them to its
    /// corresponding line index. When the instructions are parsed, all comments, comma etc, are
    /// omitted and thus the row index will correspond to the index of the instruction to the
    /// label
    pub fn generate_label_map(&mut self) -> Result<HashMap<String, usize>, ParseError> {
        let mut label_map = std::collections::HashMap::new();
        let mut row_index: usize = 0;

        for token in self.tokens.iter() {
            match &token.kind {
                TokenKind::LabelDef(name) => {
                    if label_map.contains_key(name) {
                        return Err(ParseError::DuplicateLabel {
                            label: name.clone(),
                            position: token.span.start,
                        });
                    }
                    label_map.insert(name.clone(), row_index);
                }
                TokenKind::Opcode(_) => {
                    row_index += 1;
                }
                _ => {}
            }
        }

        Ok(label_map)
    }
}

fn expected_operand_count(opcode: Opcode) -> usize {
    match opcode {
        Opcode::NOP | Opcode::END => 0,
        _ => match opcode.instruction_format() {
            crate::isa::InstrFormat::R3 => 3,
            crate::isa::InstrFormat::R2 => 2,
            crate::isa::InstrFormat::RI => 2,
            crate::isa::InstrFormat::RRI => 3,
            crate::isa::InstrFormat::RII => 3,
            crate::isa::InstrFormat::I => 1,
            crate::isa::InstrFormat::NoOP => 0,
        },
    }
}
