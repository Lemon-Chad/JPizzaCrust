
use super::token::Position;
use std::fmt::{ self, Display, Formatter };
use crate::utils::general as utils;

/// Result that either returns a lexer error or a value.
pub enum LexResult<'a, T> {
    /// Lexer result.
    Ok(T),
    /// Lexer error.
    Err(String, String, Position<'a>),
}

impl<'a, T> Display for LexResult<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok(_) => write!(f, "Ok!"),
            // If it's an error, unpack it.
            Self::Err(name, reason, pos,) => {
                // Now format it to make it look pretty :D
                write!(f, "{} Error: {}\nFile {}, line {}\n{}", name, reason, pos.filename(), 
                    utils::line_of(pos.src(), pos.index()) + 1, utils::underline_selection(pos))
            }
        }
    }
}
