mod lex;
use lex::lexer::Lexer;
use lex::token::Token;

#[allow(while_true)]
fn main() {
    let mut lexer = Lexer {
        source: "1/2-4*(5+6)".to_string(),
        .. Default::default()
    };

    while true {
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
