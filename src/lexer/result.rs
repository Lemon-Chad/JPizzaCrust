
use super::token::Position;

/// Result that either returns a lexer error or a value.
pub enum LexResult<'a, T> {
    /// Lexer result.
    Ok(T),
    /// Lexer error.
    Err(String, String, Position<'a>),
}
