use crate::lex::token::Token;
use crate::lex::token::Token::T_RPAREN;
use crate::parser::parse_error::ParseError;
use crate::parser::expressions::expression::Expression;
use crate::parser::expressions::val::Val;
use crate::parser::expressions::mult_op::MultOp;
use crate::parser::expressions::add_op::AddOp;
use crate::parser::expressions::sub_op::SubOp;
use crate::parser::expressions::div_op::DivOp;

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
        print!("\n");
        return self.parse_e();
    }

    fn parse_e(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let expression = self.parse_t();
        print!("{:?}\n", expression);
        return match expression {
            Ok(expression) => self.parse_e_dot(expression),
            _ => return Err(ParseError::FAILED),
        };
    }

    fn parse_e_dot(&mut self, expression: Box<dyn Expression>) -> Result<Box<dyn Expression>, ParseError> {
        let current = self.current_token();
        if current.is_none() { return Ok(expression); }

        let token = current.unwrap();
        return match token {
            Token::T_PLUS => { // E' = +TE'
                self.next_token();
                let right_expression = self.parse_t();

                let node: AddOp;
                match right_expression {
                    Ok(right) => node = AddOp { right, left: expression },
                    _ => return Err(ParseError::FAILED),
                }

                print!("{:?}\n", node);

                return self.parse_e_dot(Box::new(node));
            },
            Token::T_MINUS => { // E' = -TE'
                self.next_token();
                let right_expression = self.parse_t();

                let node: SubOp;
                match right_expression {
                    Ok(right) => node = SubOp { right, left: expression },
                    _ => return Err(ParseError::FAILED),
                }

                print!("{:?}\n", node);

                return self.parse_e_dot(Box::new(node));
            },
            Token::EOF | Token::T_RPAREN => Ok(expression),
            _ => Err(ParseError::FAILED),
        }
    }

    fn parse_t(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let expression = self.parse_f();
        print!("{:?}\n", expression);
        return match expression {
            Ok(expression) => self.parse_t_dot(expression),
            _ => return Err(ParseError::FAILED),
        };
    }

    fn parse_t_dot(&mut self, expression: Box<dyn Expression>) -> Result<Box<dyn Expression>, ParseError> {
        let current = self.current_token();
        if current.is_none() { return Ok(expression); }

        let token = current.unwrap();
        return match token {
            Token::T_STAR => { // T' = *FT'
                self.next_token();
                let right_expression = self.parse_f();

                let node: MultOp;
                match right_expression {
                    Ok(right) => node = MultOp { right, left: expression },
                    _ => return Err(ParseError::FAILED),
                }

                print!("{:?}\n", node);

                return self.parse_t_dot(Box::new(node));
            },
            Token::T_SLASH => { // T' = /FT'
                self.next_token();
                let right_expression = self.parse_f();

                let node: DivOp;
                match right_expression {
                    Ok(right) => node = DivOp { right, left: expression },
                    _ => return Err(ParseError::FAILED),
                }

                print!("{:?}\n", node);

                return self.parse_t_dot(Box::new(node));
            },
            Token::EOF | Token::T_RPAREN | Token::T_PLUS | Token::T_MINUS => Ok(expression),
            _ => Err(ParseError::FAILED),
        }
    }

    fn parse_f(&mut self) -> Result<Box<dyn Expression>, ParseError> {
        let current = self.current_token();

        if current.is_none() {
            return Err(ParseError::FAILED);
        }

        let token = current.unwrap();
        return match *token {
            Token::T_LPAREN => {
                self.next_token();
                let expression = self.parse_e();

                match self.current_token() {
                    Some(T_RPAREN) => expression,
                    _ => Err(ParseError::FAILED),
                }
            },
            Token::T_NUMBER(val) => {
                self.next_token();
                Ok(Box::new(Val { value: val }))
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