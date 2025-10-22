use std::collections::HashMap;

use crate::{
    errors::ParseError,
    isa::{Opcode, UnresolvedInstruction, UnresolvedOperand},
    lexer::{Lexer, Token, TokenKind},
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
    pub fn parse_instructions(&mut self) -> Result<Vec<UnresolvedInstruction>, ParseError> {
        // Entry point for parsing logic
        // At each opcode, we parse_operands and collect the opcode
        let mut instructions: Vec<UnresolvedInstruction> = Vec::new();
        let label_map = self.generate_label_map()?;
        self.set_pos(0); // reset position for actual parsing
        while let Some(token) = self.peek() {
            // We need to skip untill opcode
            match &token.kind {
                TokenKind::Opcode(opcode) => {
                    let opcode = *opcode;
                    self.next(); // consume the opcode token
                    let operands = self.parse_operands(&label_map)?;
                    instructions.push(UnresolvedInstruction { opcode, operands });
                }
                _ => {
                    self.next(); // consume non-opcode tokens
                }
            }
        }

        Ok(instructions)
    }

    /// This function is only to be called when previous token was an OPcode. It is expected that we have advanced, and the next token will be an operands
    fn parse_operands(
        &mut self,
        label_map: &HashMap<String, usize>,
    ) -> Result<Vec<UnresolvedOperand>, ParseError> {
        let mut operands: Vec<UnresolvedOperand> = Vec::with_capacity(2);

        while let Some(token) = self.peek() {
            match &token.kind {
                //TODO: Expect comma and terminator, instead of breaking on next opcode. This
                //enforces correct syntax instead of this janky solution
                TokenKind::Register(reg_num) => {
                    operands.push(UnresolvedOperand::Register(*reg_num));
                    self.next(); // consume the token
                }
                TokenKind::Immediate(imm_val) => {
                    operands.push(UnresolvedOperand::Immediate(*imm_val));
                    self.next(); // consume the token
                }
                TokenKind::LabelRef(label_name) => {
                    // Check if label exists in label_map
                    if !label_map.contains_key(label_name) {
                        return Err(ParseError::UndefinedLabel {
                            label: label_name.clone(),
                            position: token.span.start,
                        });
                    }
                    operands.push(UnresolvedOperand::LabelRef(label_name.clone()));
                    self.next(); // consume the token
                }
                TokenKind::Opcode(_) => break, // Stop parsing operands on next opcode

                _ => {
                    self.next();
                } // Skips other tokens like commas, comments, LabelRef and LabelDef
            }
        }

        // error handling
        if operands.is_empty() {
            return Err(ParseError::UnexpectedToken {
                expected: "at least one operand".to_string(),
                found: "none".to_string(),
                position: self.position,
            });
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
                    let res = label_map.insert(name.clone(), row_index);
                    //TODO: handle errors for multiple labels
                    if res.is_some() {
                        return Err(ParseError::DuplicateLabel {
                            label: name.clone(),
                            position: token.span.start,
                        });
                    }
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
