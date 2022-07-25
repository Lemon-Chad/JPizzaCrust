use std::fmt;

/// Stores the position of something in 
/// a string (src).
/// Index indicates the starting character's index,
/// and len indicates how many characters
/// the selection spans.
pub struct Position<'a> {
    index: usize,
    len: usize,
    src: &'a str,
}

impl<'a> Position<'a> {
    pub fn new(index: usize, len: usize, src: &'a str) -> Self {
        Position { index, len, src }
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
}

impl<'a> fmt::Display for Token<'a> {
    // Simply writes token as debug for to_string
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Plus(_) => "+".to_string(),
            Self::Minus(_) => "-".to_string(),
            Self::Star(_) => "*".to_string(),
            Self::Slash(_) => "/".to_string(),
            Self::Int(_, n) => n.to_string(),
            Self::Float(_, n) => format!("{}f", n),
        })
    }
}
