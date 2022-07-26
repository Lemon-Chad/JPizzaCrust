use crate::{types::{JType, utils::is_numeric, TypeResult}, lexer::token::Position};

// Enum for each expression in the tree
pub enum Expr<'a> {
    // Statements
    Body(Position<'a>, Vec<Expr<'a>>),

    // Literals
    Int(Position<'a>, i64),
    Float(Position<'a>, f64),

    // Binary operations
    Add(Position<'a>, Box<Expr<'a>>, Box<Expr<'a>>),
    Sub(Position<'a>, Box<Expr<'a>>, Box<Expr<'a>>),
    Mul(Position<'a>, Box<Expr<'a>>, Box<Expr<'a>>),
    Div(Position<'a>, Box<Expr<'a>>, Box<Expr<'a>>),
}

impl<'a> Expr<'a> {
    /// Gets the type of the expression.
    /// In the result of a TypeError, a TypeResult::Err
    /// will be returned.
    fn get_type(&self) -> TypeResult<'a, '_> {
        /*
        * Note:
        * You'll see a lot of the following snippet:
        * 
        * match res {
        *   TypeResult::Ok(t) => t,
        *   err => return err,
        * };
        * 
        * This checks if the typeresult is ok, and if not,
        * returns the error. Simple as that.
        */
        match self {
            Self::Body(_, _) => TypeResult::Ok(JType::Void),
            Self::Int(_, _) => TypeResult::Ok(JType::Int),
            Self::Float(_, _) => TypeResult::Ok(JType::Float),
            Self::Add(pos, left, right) => {
                let left_type = match left.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                let right_type = match right.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                // If they're both numbers
                if is_numeric(&left_type) && is_numeric(&right_type) {
                    // If one is a float, the outcome will be a float. 
                    // Otherwise it will be an integer.
                    return if matches!(left_type, JType::Float) || matches!(right_type, JType::Float) {
                        TypeResult::Ok(JType::Float)
                    } else {
                        TypeResult::Ok(JType::Int)
                    };
                }
                // If it's not a found expression, crash!
                TypeResult::Err(
                    "TypeMismatch".to_string(), 
                    format!("You cannot add '{}' and '{}'", left_type, right_type), 
                    pos
                )
            },
            Self::Sub(pos, left, right) => {
                let left_type = match left.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                let right_type = match right.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                // If they're both numbers
                if is_numeric(&left_type) && is_numeric(&right_type) {
                    // If one is a float, the outcome will be a float. 
                    // Otherwise it will be an integer.
                    return if matches!(left_type, JType::Float) || matches!(right_type, JType::Float) {
                        TypeResult::Ok(JType::Float)
                    } else {
                        TypeResult::Ok(JType::Int)
                    };
                }
                // If it's not a found expression, crash!
                TypeResult::Err(
                    "TypeMismatch".to_string(), 
                    format!("You cannot add '{}' and '{}'", left_type, right_type), 
                    pos
                )
            },
            Self::Mul(pos, left, right) => {
                let left_type = match left.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                let right_type = match right.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                // If they're both numbers
                if is_numeric(&left_type) && is_numeric(&right_type) {
                    // If one is a float, the outcome will be a float. 
                    // Otherwise it will be an integer.
                    return if matches!(left_type, JType::Float) || matches!(right_type, JType::Float) {
                        TypeResult::Ok(JType::Float)
                    } else {
                        TypeResult::Ok(JType::Int)
                    };
                }
                // If it's not a found expression, crash!
                TypeResult::Err(
                    "TypeMismatch".to_string(), 
                    format!("You cannot add '{}' and '{}'", left_type, right_type), 
                    pos
                )
            },
            Self::Div(pos, left, right) => {
                let left_type = match left.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                let right_type = match right.get_type() {
                    TypeResult::Ok(t) => t,
                    err => return err,
                };
                // If they're both numbers
                if is_numeric(&left_type) && is_numeric(&right_type) {
                    // If one is a float, the outcome will be a float. 
                    // Otherwise it will be an integer.
                    return if matches!(left_type, JType::Float) || matches!(right_type, JType::Float) {
                        TypeResult::Ok(JType::Float)
                    } else {
                        TypeResult::Ok(JType::Int)
                    };
                }
                // If it's not a found expression, crash!
                TypeResult::Err(
                    "TypeMismatch".to_string(), 
                    format!("You cannot add '{}' and '{}'", left_type, right_type), 
                    pos
                )
            },
        }
    }

    /// Returns the position of the expression.
    fn pos(&self) -> &Position<'a> {
        // Literally just arms for every variant to get the position
        match self {
            Self::Body(pos, _) => pos,

            // Literals
            Self::Float(pos, _) => pos,
            Self::Int(pos, _) => pos,

            // Operations
            Self::Add(pos, _, _) => pos,
            Self::Sub(pos, _, _) => pos,
            Self::Mul(pos, _, _) => pos,
            Self::Div(pos, _, _) => pos,
        }
    }
}
