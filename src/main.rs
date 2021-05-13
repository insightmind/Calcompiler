mod lex;
mod parser;

use crate::lex::lexer::Lexer;
use crate::parser::parser::Parser;

fn main() {
    let mut lexer = Lexer {
        source: "3.0+.05e2/2.0-4.0*(500.0e-3L+6.0e+1f)".to_string(),
        .. Default::default()
    };

    let mut parser;
    match lexer.lex_source() {
        Ok(stream) => parser = Parser { token_stream: stream, token_index: 0 },
        Err(err) => panic!("{}", err),
    }

    let _expresssionTree =

    match parser.parse() {
        Ok(mut tree) => {
            print!("{:?}\n", tree);
            print!("{:?}\n", tree.evaluate());
        },
        Err(err) => panic!("{}", err),
    };
}
