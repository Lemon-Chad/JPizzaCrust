use std::fmt::{ self, Display, Formatter };


// Enum for each type in the language
pub enum JType {
    Int,
    Float,
    Void,
}

impl Display for JType {
    // Type as a string
    // Very nice
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Int => "int",
            Self::Float => "float",
            Self::Void => "void",
        })
    }
}
