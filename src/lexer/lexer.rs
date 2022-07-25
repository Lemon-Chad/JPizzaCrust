
use super::token::{ Token, Position };
use super::result::LexResult;
use crate::utils::general::is_some_and;

struct Lexer<'a> {
    src: &'a str,
    chars: Vec<char>,
    index: usize,
    current: Option<char>,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        let chars: Vec<char> = src.chars().collect();
        let current = Some(chars[0]);
        Self {
            src,
            chars,
            index: 0,
            current,
        }
    }

    /// Steps to the next character in the source code.
    fn advance_once(&mut self) {
        self.index += 1;
        self.current = if self.index < self.chars.len() {
            Some(self.chars[self.index])
        } else {
            None
        };
    }

    /// Steps to the next character in the source code
    /// and skips whitespace
    fn advance(&mut self) {
        loop {
            self.advance_once();
            if !is_some_and(&self.current, |x| x.is_whitespace()) {
                break;
            }
        }
    }

    /// Lexes an identifier or keyword
    fn identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current {
            if !c.is_alphanumeric() {
                break;
            }
            // Append to identifier
            ident.push(c);
            self.advance();
        }
        ident
    }

    /// Lexes a number token
    fn number(&mut self, index: usize) -> LexResult<'a, Token<'a>> {
        let mut num = String::new();
        let mut dots = 0;
        let mut is_float = false;
        let mut is_hex = false;
        while let Some(c) = self.current {
            // Hellish method to get lowercase character.
            let lowercase_c = c.to_lowercase().next().unwrap();
            // If the number ends with 'f', then it's a float.
            if lowercase_c == 'f' && !is_hex {
                is_float = true;
                self.advance();
                break;
            }
            // If there's an x, it might be hex!
            if lowercase_c == 'x' && !is_hex {
                // If the number is not a 0, it's not hex
                // Format for hex is 0xABCD so the only character preceding it should be a 0
                if num != "0" {
                    break;
                }
                is_hex = true;
                self.advance();
                // Reset number string because 0x causes a formatting error.
                num = String::new();
                continue;
            }
            // If it's not a number or a dot
            // and if it's hexadecimal but not in the character range
            // then break.
            if (!('a'..='f').contains(&lowercase_c) || !is_hex) && !c.is_numeric() && c != '.' {
                break;
            }
            // Search for decimal points
            if c == '.' {
                // If the number is also hexadecimal, uh oh!
                if is_hex {
                    return LexResult::Err("NumberFormat".to_string(), "
                        Hex number cannot be a float.".to_string(), self.pos(self.index));
                }
                dots += 1;
                // If there's a decimal point, it must be a floating point number!
                is_float = true;
                // If there is more than 1 dot (eg 12.34.) then break
                if dots > 1 {
                    break;
                }
            }
            // Append to number
            num.push(c);
            self.advance();
        }

        // If the last character is a dot, then that's a syntax error.
        if is_some_and(&num.chars().last(), |x| *x == '.') {
            return LexResult::Err("NumberFormat".to_string(), 
                "Expected number after decimal point.".to_string(), self.pos(self.index));
        }

        // If there's no number then uhhh oh no!!
        if num.len() == 0 {
            return LexResult::Err("NumberFormat".to_string(), 
                "Expected number.".to_string(), self.pos(self.index));
        }

        // Parse number string as f64 if it is a float
        // Force unwrap because if it crashes, the lexer is broken
        LexResult::Ok(if is_float {
            let n: f64 = num.parse().unwrap();
            Token::Float(self.pos(index), n)
        } else {
            // Otherwise do same with i64
            let n: i64 = if is_hex {
                // If it's hexadecimal, parse in base 16.
                i64::from_str_radix(&num, 16).unwrap()
            } else {
                num.parse().unwrap()
            };
            Token::Int(self.pos(index), n)
        })
    }

    /// Returns a position object ranging from a given 
    /// start index to the current character
    fn pos(&self, index: usize) -> Position<'a> {
        Position::new(index, self.index - index, self.src)
    }

    /// Returns a LexResult::Ok of the token and advances to
    /// the next character. Useful for one line token returns.
    fn tok(&mut self, token: Token<'a>) -> LexResult<'a, Token<'a>> {
        self.advance();
        LexResult::Ok(token)
    }

    fn gather_token(&mut self) -> LexResult<'a, Token<'a>> {
        let index = self.index;
        if let Some(c) = self.current {
            match c {
                // Basic one character tokens
                '+' => self.tok(Token::Plus(self.pos(index))),
                '-' => self.tok(Token::Minus(self.pos(index))),
                '*' => self.tok(Token::Star(self.pos(index))),
                '/' => self.tok(Token::Slash(self.pos(index))),
                // If it's a number, generate number token
                _ if c.is_numeric() => self.number(index),
                // Otherwise, unknown token
                _ => LexResult::Err("UnknownToken".to_string(), "Unknown symbol or token".to_string(),
                    self.pos(index)),
            }
        } else {
            LexResult::Err("EndOfFile".to_string(), "Unexpected end of file.".to_string(), 
                self.pos(index))
        }
    }

    fn lex(&mut self) -> LexResult<'a, Vec<Token<'a>>> {
        let mut tokens = Vec::new();
        // While the current character exists (not end of file)
        while self.current.is_some() {
            // Attempt to lex a token
            let res = self.gather_token();
            match res {
                // Append to token vector if success
                LexResult::Ok(token) => tokens.push(token),
                // Return error otherwise
                LexResult::Err(head, tail, pos) => return LexResult::Err(head, tail, pos),
            };
        }
        LexResult::Ok(tokens)
    }

}

/// Takes in source code and creates tokens from it
pub fn lex<'a>(src: &'a str) -> LexResult<'a, Vec<Token<'a>>> {
    let mut lexer = Lexer::new(src);
    lexer.lex()
}
