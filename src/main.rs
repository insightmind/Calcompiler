mod lex;
use lex::lexer::Lexer;
use lex::token::Token;

fn main() {
    let mut lexer = Lexer {
        source: ".05e10/2.0-4.0*(500.075e-3L+6.0e+4f)".to_string(),
        .. Default::default()
    };

    loop {
        let token_result = lexer.next_token();
        match token_result {
            Ok(token) => {
                print!("<{}>", token);

                if token == Token::EOF { break; }
            },
            Err(error) => print!("{}", error),
        }
    }
}
