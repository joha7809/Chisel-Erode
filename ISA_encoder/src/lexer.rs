use std::{fmt::Display, io::Write};

use crate::isa::{self, Opcode, Operand, REGISTER_LIMIT};

/// Represents one lexical unit (token) in the assembly code.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Opcode(Opcode),
    Register(usize),
    Immediate(i32),
    LabelDef(String), // e.g., "loop:" defines a label
    LabelRef(String), // e.g., used in a jump instruction
    Comma,
    Comment(String),
    Terminator,
}

/// A token instance with its type and position in the input string.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

// Implement readable debuf display for Token
impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            TokenKind::Opcode(op) => write!(f, "Opcode({:?})", op),
            TokenKind::Register(reg) => write!(f, "Register(R{})", reg),
            TokenKind::Immediate(imm) => write!(f, "Immediate({})", imm),
            TokenKind::LabelDef(label) => write!(f, "LabelDef({})", label),
            TokenKind::LabelRef(label) => write!(f, "LabelRef({})", label),
            TokenKind::Comma => write!(f, "Comma(,)"),
            TokenKind::Comment(comment) => write!(f, "Comment(# {})", comment),
            TokenKind::Terminator => write!(f, "Terminator(;)"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Span {
    start: usize,
    end: usize,
    line: usize,
}

impl From<(usize, usize, usize)> for Span {
    fn from(tuple: (usize, usize, usize)) -> Span {
        Span {
            start: tuple.0,
            end: tuple.1,
            line: tuple.2,
        }
    }
}

pub struct Lexer<'a> {
    // Returns a vector of tokens (Opcode and Operands) from the input stringin
    input: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
    line: usize,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            chars: input.char_indices().peekable(),
            line: 0,
            position: 0,
        }
    }

    pub fn lex(mut self) -> Vec<Token> {
        let mut tokens = Vec::with_capacity(16);
        while let Some(tok) = self.next_token() {
            tokens.push(tok);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();

        let (start, c) = *self.chars.peek()?;
        self.position = start;

        let kind = match c {
            '#' => return Some(self.parse_comment(start)),
            ',' => TokenKind::Comma,
            c if c.is_ascii_digit() || c == '-' => return Some(self.parse_number(start)),
            ';' => TokenKind::Terminator,
            c if c.is_alphabetic() => return Some(self.branch_identifier(start, c)),
            _ => unreachable!(), // skip unknowns
        };
        // Branches that hasnt returned yet are simple tokens, so consume the chars
        self.chars.next();

        Some(Token {
            kind,
            span: (start, start + 1, self.line).into(),
        })
    }

    fn skip_whitespace(&mut self) {
        while let Some((_, c)) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
            } else {
                break;
            }
        }
    }

    fn parse_comment(&mut self, start: usize) -> Token {
        let comment: String = self
            .chars
            .by_ref()
            .take_while(|(_, c)| *c != '\n')
            .map(|(_, c)| c)
            .collect();
        let len = comment.chars().count();
        Token {
            kind: TokenKind::Comment(comment),
            span: (start, start + len, self.line).into(),
        }
    }

    /// Parse a register token starting with 'R' followed by digits. E.g., "R1", "R15"
    /// Is is assumed that the 'R' has already been consumed.
    fn parse_register(&mut self, start: usize) -> Token {
        // Safely consume 'R'
        let r = self.chars.next();
        debug_assert!(r.is_some() && r.unwrap().1 == 'R');
        let mut digits: String = String::with_capacity(2);

        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_ascii_digit() {
                digits.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let num = digits.parse::<usize>().unwrap();
        if num == 0 || num > REGISTER_LIMIT {
            // TODO: return error token or panic
        }

        Token {
            kind: TokenKind::Register(num),
            span: (start, start + digits.len(), self.line).into(),
        }
    }
    /// Parse a number token, which can be a positive or negative integer.
    /// It is assumed that the first digit or '-' has already been peeked, but not consumed.
    fn parse_number(&mut self, start: usize) -> Token {
        let mut number = String::with_capacity(4);
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_ascii_digit() {
                number.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let value = number.parse::<i32>().unwrap();
        Token {
            kind: TokenKind::Immediate(value),
            span: (start, start + number.len(), self.line).into(),
        }
    }

    /// Decide if the identifier is a register or a label/opcode, and parse accordingly.
    /// It is assumed that the first character has already been peeked, but not consumed.
    fn branch_identifier(&mut self, start: usize, first: char) -> Token {
        let mut lookahead = self.chars.clone();
        lookahead.next(); // consume first char in lookahead

        if first == 'R'
            && let Some((_, c)) = lookahead.next()
            && c.is_ascii_digit()
        {
            return self.parse_register(start);
        }
        self.parse_identifier(start, first)
    }

    /// Parse an identifier which can be an opcode, label definition, or label reference.
    /// It is assumed that the first character has been peeked and consumed.
    fn parse_identifier(&mut self, start: usize, first: char) -> Token {
        // Safely advance
        self.chars.next();

        let mut ident = String::from(first);
        // Read alphanumeric + underscore only
        while let Some(&(_, c)) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' || c == ':' {
                ident.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        let len = ident.len();

        let kind = if ident.ends_with(':') {
            TokenKind::LabelDef(ident.trim_end_matches(':').to_string())
        } else if let Some(opcode) = isa::Opcode::from_str(&ident) {
            TokenKind::Opcode(opcode)
        } else {
            TokenKind::LabelRef(ident)
        };

        Token {
            kind,
            span: (start, start + len, self.line).into(),
        }
    }
}
