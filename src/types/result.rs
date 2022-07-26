use crate::lexer::token::Position;
use crate::utils::general as utils;
use std::fmt::{ self, Formatter, Display };
use super::JType;

pub enum TypeResult<'a, 'b> {
    Ok(JType),
    // In case there's a TypeError
    Err(String, String, &'b Position<'a>),
}

// Error displaying for TypeResults
impl<'a, 'b> Display for TypeResult<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ok(t) => write!(f, "{}", t),
            // If it's an error, unpack it.
            Self::Err(name, reason, pos,) => {
                // Now format it to make it look pretty :D
                write!(f, "{} Error: {}\nFile {}, line {}\n{}", name, reason, pos.filename(), 
                    utils::line_of(pos.src(), pos.index()) + 1, utils::underline_selection(pos))
            }
        }
    }
}
