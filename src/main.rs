mod lexer;
mod utils;
use utils::logging;

fn main() {
    let demo_code = "1. + 2 / 3 - 4 * 5";
    let result = lexer::lexer::lex("demo_code".to_string(), demo_code);
    logging::println(&format!("Demo code: {}", demo_code));
    if let lexer::result::LexResult::Ok(tokens) = result {
        let mut s = "[ ".to_string();
        for token in tokens {
            let token_str = token.to_string();
            s.push_str(&token_str);
            s.push_str(", ");
        }
        s.pop();
        s.pop();
        s.push_str(" ]");
        logging::println(&format!("Tokens: {}", s));
    }
}
