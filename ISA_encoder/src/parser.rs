use std::collections::HashMap;

use crate::{
    isa::Opcode,
    lexer::{Lexer, Token, TokenKind},
};

pub struct Parser {
    tokens: Vec<Token>,
    label_map: std::collections::HashMap<String, usize>,
    position: usize,
}
/// Returns a vec of instructions.
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
            label_map: HashMap::new(),
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

    fn expect(&mut self, kind: &TokenKind) -> Result<&Token, String> {
        match self.peek() {
            Some(token) if &token.kind == kind => {
                self.next().ok_or("SHOULD BE UNREACHABLE".to_string())
            }
            Some(token) => Err(format!("Expected {:?}, found {:?}", kind, token.kind)),
            None => Err("Unexpected end of input".to_string()),
        }
    }

    // We need to handle label logic cleverly
    // Which datatype can we use for key of str and value of index of the label instructions
    pub fn parse(&mut self) {
        // Entry point for parsing logic
        // Example: while let Some(token) = self.peek() { ... }
        while let Some(token) = self.peek() {}
    }

    fn parse_op_code(self, token: &Token) {
        if let TokenKind::Opcode(ref code) = token.kind {
            let args = code.arg_types();
            for arg in args {}
        }
    }

    fn generate_label_map(&mut self) -> HashMap<String, usize> {
        let mut label_map = std::collections::HashMap::new();
        let mut row_index: usize = 0;

        for token in self.tokens.iter() {
            match &token.kind {
                TokenKind::LabelDef(name) => {
                    label_map.insert(name.clone(), row_index);
                    //TODO: handle errors for multiple labels
                }
                TokenKind::Opcode(_) => {
                    row_index += 1;
                }
                _ => {}
            }
        }

        label_map
    }
}
