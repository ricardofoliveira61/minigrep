use crate::search::{SearchLine, SearchResult};
use colored::*;
use std::fmt::Write;

pub fn show_match(pattern_match: &SearchResult, show_file: bool, show_number: bool) {
    for line in &pattern_match.lines {
        let mut output = String::new();

        // show file name in magenta
        if show_file {
            write!(output, "{}: ", pattern_match.file_name.magenta()).unwrap();
        }

        //show line number in green
        if show_number {
            write!(output, "{}: ", line.get_line_number().to_string().green()).unwrap();
        }

        match line {
            SearchLine::ContextLine { content, .. } => {
                output.push_str(content);
            }
            SearchLine::Match {
                content,
                start,
                end,
                ..
            } => {
                let before = &content[..*start];
                let pattern = &content[*start..*end];
                let after = &content[*end..];

                write!(output, "{}{}{}", before, pattern.red().bold(), after).unwrap();
            }
        }

        println!("{}", output);
    }
}

// show the number of matches per file
pub fn show_count(file_name: &str, count: usize) {
    println!("{}:{}", file_name.magenta(), count.to_string().green());
}
