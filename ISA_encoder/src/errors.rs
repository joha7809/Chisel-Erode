// Make some errors for our parser
#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: String,
        position: usize,
    },
    UnexpectedEndOfInput,
    DuplicateLabel {
        label: String,
        position: usize,
    },
    UndefinedLabel {
        label: String,
        position: usize,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Parse Error at position {}: Expected {}, found {}",
                    position, expected, found
                )
            }
            ParseError::UnexpectedEndOfInput => {
                write!(f, "Parse Error: Unexpected end of input",)
            }
            ParseError::DuplicateLabel { label, position } => {
                write!(
                    f,
                    "Parse Error at position {}: Duplicate label '{}'",
                    position, label
                )
            }
            ParseError::UndefinedLabel { label, position } => {
                write!(
                    f,
                    "Parse Error at position {}: Undefined label '{}'",
                    position, label
                )
            }
        }
    }
}
