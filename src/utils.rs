
/// Returns true if the given option
/// has a value AND the callback
/// returns true.
pub fn is_some_and<T>(option: &Option<T>, func: fn(&T) -> bool) -> bool {
    if let Some(t) = option {
        func(t)
    } else {
        false
    }
}
