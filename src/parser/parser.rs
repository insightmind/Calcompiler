use crate::lex::token::Token;
use crate::parser::expressions::expression::Expression;
use crate::parser::expressions::val::Val;
use crate::parser::parse_error::ParseError;
use crate::lex::token::Token::T_RPAREN;

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
    pub token_stream: Vec<Token>,
    pub token_index: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn parse(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        return self.parse_e();
    }

    fn parse_e(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        return Err(ParseError::FAILED);
    }

    fn parse_e_dot(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        return Err(ParseError::FAILED);
    }

    fn parse_t(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        return Err(ParseError::FAILED);
    }

    fn parse_t_dot(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        return Err(ParseError::FAILED);
    }

    fn parse_f(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let current = self.current_token();

        if current.is_none() {
            return Err(ParseError::FAILED);
        }

        let token = current.unwrap();
        return match token {
            Token::T_LPAREN => {
                self.next_token();
                let expression = self.parse_e();
                self.next_token();

                match self.current_token() {
                    Some(T_RPAREN) => expression,
                    _ => Err(ParseError::FAILED),
                }
            },
            Token::T_NUMBER(val) => {
                Ok(Box::new(Val { value: *val }))
            },
            _ => Err(ParseError::FAILED),
        }
    }

    fn current_token(&mut self) -> Option<&Token> {
        return self.token_stream.get(self.token_index);
    }

    fn next_token(&mut self) {
        self.token_index += 1;
    }
}