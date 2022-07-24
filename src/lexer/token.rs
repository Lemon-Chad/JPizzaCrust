
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

/// Tokens produced by the lexer.
/// Every token stores at least a position.
pub enum Token {
    Plus(Position),
    Minus(Position),
    Star(Position),
    Slash(Position),
    Number(Position, i32),
}
