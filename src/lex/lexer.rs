use super::lex_error::LexError;
use super::token::Token;
use super::token::Token::T_NUMBER;

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
        let mut float_string = String::from("");
        let mut current_char = self.current_char();
        // Simple state machine as on slide 35
        let mut state = 0;

        'lex_loop: while current_char.is_some() {
            let character = current_char.unwrap();

            match state {
                0 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                            state = 2;
                        },
                        '.' => {
                            float_string.push(character);
                            state = 1;
                        },
                        _ => break 'lex_loop,
                    }
                }
                1 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                            state = 3;
                        },
                        _ => break 'lex_loop,
                    }
                },
                2 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                        },
                        '.' => {
                            float_string.push(character);
                            state = 3;
                        },
                        _ => break 'lex_loop,
                    }
                },
                3 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                        },
                        'e' | 'E' => {
                            float_string.push(character);
                            state = 4;
                        },
                        'f' | 'F' | 'l' | 'L' => {
                            float_string.push(character);
                            state = 7;
                        },
                        _ => break 'lex_loop,
                    }
                },
                4 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                            state = 6;
                        },
                        '-' | '+' => {
                            float_string.push(character);
                            state = 5;
                        },
                        _ => break 'lex_loop,
                    }
                },
                5 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                            state = 6;
                        },
                        _ => break 'lex_loop,
                    }
                },
                6 => {
                    match character {
                        c if { c.is_ascii_alphanumeric() } => {
                            float_string.push(c);
                        },
                        'f' | 'F' | 'l' | 'L' => {
                            float_string.push(character);
                            state = 7;
                        },
                        _ => break 'lex_loop,
                    }
                },
                _ => break 'lex_loop,
            }

            self.next_char();
            current_char = self.current_char();
        }

        self.next_char();

        // FIX: Implement actually parsing
        // Some floats from the automata of the lecture are not
        // supported by parse::<f64>
        return match state {
            3 | 6 | 7 => {
                match float_string.parse::<f64>() {
                    Ok(val) => return Ok(T_NUMBER(val)),
                    Err(err) => Err(LexError::FLOAT_PARSE_FAIL(err)),
                }
            },
            _ => Err(LexError::UNKNOWN_FLOAT),
        }
    }

    fn current_char(&mut self) -> Option<char> {
        return self.source.chars().nth(self.char_index);
    }

    fn next_char(&mut self) {
        self.char_index += 1;
    }
}