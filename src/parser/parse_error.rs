use std::fmt;

#[allow(non_camel_case_types, dead_code)]
#[derive(Debug, PartialEq)]
pub enum ParseError {
    FAILED
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            ParseError::FAILED => write!(f, "Parsing failed"),
        };
    }
}