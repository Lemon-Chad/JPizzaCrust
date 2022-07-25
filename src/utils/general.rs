use crate::lexer::token::Position;
use std::cmp::min;
use locale_codes;

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

/// Gets the line number that the character
/// at the given index is located on.
pub fn line_of(src: &str, index: usize) -> usize {
    src[0..=index].matches('\n').count()
}

pub fn underline_selection(pos: &Position) -> String {
    // Determines the underline characters
    // based on whether or not UTF-8 can be used.
    let (left_pipe, underscore, right_pipe) = 
        if locale_codes::codeset::all_names().contains(&String::from("UTF-8")) {
            ('╰', '─', '╯')
        } else {
            ('\\', '_', '/')
    };

    let index = pos.index();
    let len = pos.len();
    // See what I did there? ENDex? Get it?
    // Did you get the joke? I thought it was funny.
    // Laugh.
    let endex = index + len;

    // Offset before starting highlighting the first line
    let mut start_offset = 0;
    // Offset before ending highlighting the last line
    let mut end_offset = 0;
    let mut i = 0;
    let mut lines: Vec<&str> = Vec::new();
    for line in pos.src().split('\n') {
        if i <= index && index < i + line.len() {
            start_offset = index - i;
        }
        if i <= endex && endex < i + line.len() {
            end_offset = i + line.len() - index - len;
        }

        if i <= index && endex < i + line.len() {
            lines.push(line);
        } 
        
        if endex >= i + line.len() {
            break;
        }
        
        i += line.len();
    }

    /*
    * Here we determine how much whitespace we
    * can chop off the left side. This is so that
    * indented programs don't have error messages
    * flying way off to the right side of the terminal.
    */
    let mut indent_level = usize::MAX;
    for line in &lines {
        indent_level = min(indent_level, line
            .chars()
            .take_while(|x| x.is_whitespace())
            .count()
        );
    }

    // Time to finally build the error.
    let mut s = String::new();
    for (n, l) in lines.iter().enumerate() {
        // Cut off trailing whitespace
        let mut line = &l[indent_level..];
        line = line.trim_end();
        // Add the current line
        s.push_str(line);
        s.push('\n');

        // Get the start index if it's the first line
        let start_index = if n == 0 {
            start_offset - indent_level
        } else {
            0
        };
        // Get the end index if it's the last
        let end_index = if n == lines.len() - 1 {
            l.len() - end_offset - indent_level
        } else {
            l.len()
        };

        // Add n spaces to move to the start index
        s.push_str(&" ".repeat(start_index));

        // Calculate how far the underline spans for the given line
        let span = end_index - start_index + 1;
        if span == 1 {
            // If it's only one character, add an arrow
            s.push('^');
        } else {
            // Otherwise, add a fancy underscore, like \___/ or ╰───╯ 
            // depending on whether UTF-8 works or not
            s.push(left_pipe);
            s.push_str(&underscore.to_string().repeat(span - 2));
            s.push(right_pipe);
        }
        s.push('\n');
    }
    // Return :D
    s
}
