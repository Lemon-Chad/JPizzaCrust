use std::fmt;

/// Stores the position of something in 
/// a string (src).
/// Index indicates the starting character's index,
/// and len indicates how many characters
/// the selection spans.
pub struct Position {
    index: u32,
    len: u32,
    src: &str,
}

impl Position {
    // Some basic getters for each property.
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn src(&self) -> &str {
        self.src
    }
}

impl fmt::Display for Position {
    // Writes position as {index:len} for to_string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{}:{}}", index, len)
    }
}

/// Tokens produced by the lexer.
/// Every token stores at least a position.
pub enum Token {
    Plus(Position),
    Minus(Position),
    Star(Position),
    Slash(Position),
    Number(Position, i32),
}

impl fmt::Display for Token {
    // Simply writes token as debug for to_string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
