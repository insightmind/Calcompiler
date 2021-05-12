use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum LexError {
    UNKNOWN_TOKEN(char),
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            LexError::UNKNOWN_TOKEN(c) => write!(f, "UNKNOWN_TOKEN({})", c),
        };
    }
}