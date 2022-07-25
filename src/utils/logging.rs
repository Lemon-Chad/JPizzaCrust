
/// Logs the msg (+ newline)
pub fn println(msg: &str) {
    println!("{}", msg);
}

/// Logs the msg
pub fn print(msg: &str) {
    print!("{}", msg);
}

/// Logs the msg to stderr
pub fn err(msg: &str) {
    eprintln!("{}", msg);
}
