use super::lex_error::LexError;
use super::token::Token;

// Lexer definition
pub struct Lexer {
    pub source: String,
    pub char_index: usize
}

// Default values for Lexer instantiation
impl Default for Lexer {
    fn default() -> Self {
        return Lexer {
            source: "".to_string(),
            char_index: 0,
        }
    }
}

// Lexer methods
impl Lexer {
    pub fn next_token(&mut self) -> Result<Token, LexError> {
        let char = self.current_char();

        if char.is_none() {
            return Ok(Token::EOF);
        }

        let character = char.unwrap();
        let token_result = match character {
            '+' => {
                self.next_char();
                Ok(Token::T_PLUS)
            },
            '-' => {
                self.next_char();
                Ok(Token::T_MINUS)
            },
            '*' => {
                self.next_char();
                Ok(Token::T_STAR)
            },
            '/' => {
                self.next_char();
                Ok(Token::T_SLASH)
            },
            '(' => {
                self.next_char();
                Ok(Token::T_LPAREN)
            },
            ')' => {
                self.next_char();
                Ok(Token::T_RPAREN)
            },
            c if { c.is_ascii_alphanumeric() || c == '.' } => self.lex_float(),
            '\n' | ' ' | '\t' | 'r' => { // Parse any new line or whitespace related character.
                self.next_char();
                Ok(Token::EOF)
            },
            c => Err(LexError::UNKNOWN_TOKEN(c)),
        };

        return token_result
    }

    pub fn lex_float(&mut self) -> Result<Token, LexError> {
        self.next_char();
        return Ok(Token::T_NUMBER(0f64));
    }

    fn current_char(&mut self) -> Option<char> {
        return self.source.chars().nth(self.char_index);
    }

    fn next_char(&mut self) {
        self.char_index += 1;
    }
}