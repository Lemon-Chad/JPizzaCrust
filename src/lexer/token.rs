use std::fmt;

/// Stores the position of something in 
/// a string (src).
/// Index indicates the starting character's index,
/// and len indicates how many characters
/// the selection spans.
pub struct Position<'a> {
    index: u32,
    len: u32,
    src: &'a str,
}

impl<'a> Position<'a> {
    pub fn new(index: u32, len: u32, src: &'a str) -> Self {
        Position { index, len, src }
    }

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

impl<'a> fmt::Display for Position<'a> {
    // Writes position as {index:len} for to_string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{}:{}}}", self.index, self.len)
    }
}

/// Tokens produced by the lexer.
/// Every token stores at least a position.
pub enum Token<'a> {
    Plus(Position<'a>),
    Minus(Position<'a>),
    Star(Position<'a>),
    Slash(Position<'a>),
    Number(Position<'a>, i32),
}

impl<'a> fmt::Display for Token<'a> {
    // Simply writes token as debug for to_string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
