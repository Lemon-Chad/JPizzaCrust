use super::JType;

/// Returns if a type is a numeric type,
/// aka either a float or int.
pub fn is_numeric<'a>(t: &JType) -> bool {
    return matches!(t, JType::Float) || matches!(t, JType::Int);
}
