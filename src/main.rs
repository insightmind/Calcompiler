mod lex;
mod parser;

use lex::lexer::Lexer;

fn main() {
    let mut lexer = Lexer {
        source: ".05e10/2.0-4.0*(500.075e-3L+6.0e+4f)".to_string(),
        .. Default::default()
    };

    let _token_stream = lexer.lex_source();
}
