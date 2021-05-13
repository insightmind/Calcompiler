use crate::lex::token::Token;

// The parser should verify following grammar and create a AST for it:
//
// E = T | E';
// E'= +TE' | -TE' | "";
// T = FT';
// T'= *FT' | /FT' | "";
// F = (E) | number
//
// An SLL(1) parser should do the job here.
#[allow(dead_code)]
pub struct Parser {
    pub token_stream: Vec<Token>
}