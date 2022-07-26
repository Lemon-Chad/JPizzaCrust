use std::fmt::{ self, Display, Formatter };
use std::cmp::{ min, max };

/// Stores the position of something in 
/// a string (src, filename).
/// Index indicates the starting character's index,
/// and len indicates how many characters
/// the selection spans.
pub struct Position<'a> {
    index: usize,
    len: usize,
    src: &'a str,
    filename: &'a str,
}

impl<'a> Position<'a> {
    pub fn new(index: usize, len: usize, src: &'a str, filename: &'a str) -> Self {
        Position { index, len, src, filename }
    }

    /// Allows you to add two positions together to get
    /// the area covered by both
    pub fn extend(&self, other: &Position<'a>) -> Position<'a> {
        let index = min(other.index, self.index);
        let len = max(self.index + self.len, other.index + other.len) - index;
        Position::new(index, len, self.src, self.filename)
    }

    // Some basic getters for each property.
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn src(&self) -> &str {
        self.src
    }

    pub fn filename(&self) -> &str {
        self.filename
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
    Int(Position<'a>, i64),
    Float(Position<'a>, f64),
    Identifier(Position<'a>, String),
    Keyword(Position<'a>, String),
}

impl<'a> Display for Token<'a> {
    // Simply writes token as debug for to_string
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Plus(_) => "+".to_string(),
            Self::Minus(_) => "-".to_string(),
            Self::Star(_) => "*".to_string(),
            Self::Slash(_) => "/".to_string(),
            Self::Int(_, n) => n.to_string(),
            Self::Float(_, n) => format!("{}f", n),
            Self::Identifier(_, ident) => ident.to_string(),
            Self::Keyword(_, keyword) => format!("KEYWORD:{}", keyword),
        })
    }
}
