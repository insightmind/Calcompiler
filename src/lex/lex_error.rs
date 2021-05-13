use std::fmt;
use std::num::ParseFloatError;

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, PartialEq)]
pub enum LexError {
    UNKNOWN_TOKEN(char),
    UNKNOWN_FLOAT,
    FLOAT_PARSE_FAIL(ParseFloatError)
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            LexError::UNKNOWN_TOKEN(c) => write!(f, "UNKNOWN_TOKEN({})", c),
            LexError::UNKNOWN_FLOAT => write!(f, "UNKNOWN_FLOAT"),
            LexError::FLOAT_PARSE_FAIL(err) => write!(f, "{}", err),
        };
    }
}