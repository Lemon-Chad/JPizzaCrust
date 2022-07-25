
use super::token::{ Token, Position };
use super::result::LexResult;
use super::super::utils;

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
            if !utils::is_some_and(&self.current, |x| x.is_whitespace()) {
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
    fn number(&mut self, index: usize) -> Token<'a> {
        let mut num = String::new();
        let mut dots = 0;
        while let Some(c) = self.current {
            if !c.is_numeric() && c != '.' {
                break;
            }
            if c == '.' {
                dots += 1;
                // If there is more than 1 dot (eg 12.34.) then break
                if dots > 1 {
                    break;
                }
            }
            // Append to number
            num.push(c);
            self.advance();
        }
        // Parse number string as i64 if there is no decimal point
        // Force unwrap because if it crashes, the lexer is broken
        if dots == 0 {
            let n = num.parse::<i64>().unwrap();
            Token::Int(self.pos(index), n)
        } else {
            // Otherwise do same with f64
            let n = num.parse::<f64>().unwrap();
            Token::Float(self.pos(index), n)
        }
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
                _ if c.is_numeric() => LexResult::Ok(self.number(index)),
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
