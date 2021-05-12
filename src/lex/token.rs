use std::fmt;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Token {
    // Token for `+` addition
    T_PLUS,
    // Token for `-` subtraction
    T_MINUS,
    // Token for `*` multiplication
    T_STAR,
    // Token for `/` division
    T_SLASH,
    // Token for opening parentheses `(`
    T_LPAREN,
    // Token for closing parentheses `)`
    T_RPAREN,
    // Token for constant float 64 number
    T_NUMBER(f64),
    // Token for EndOfFile
    EOF
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Token::T_PLUS => write!(f, "T_PLUS"),
            Token::T_MINUS => write!(f, "T_MINUS"),
            Token::T_STAR => write!(f, "T_STAR"),
            Token::T_SLASH => write!(f, "T_SLASH"),
            Token::T_LPAREN => write!(f, "T_LPAREN"),
            Token::T_RPAREN => write!(f, "T_RPAREN"),
            Token::T_NUMBER(val) => write!(f, "T_NUMBER({})", val),
            Token::EOF => write!(f, "EOF"),
        };
    }
}