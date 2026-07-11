use crate::search::Match;
use colored::*;

pub fn show_match(pattern_match: &Match, show_file: bool, show_number: bool) {
    let mut output = String::new();

    // show file name in magenta
    if show_file {
        output.push_str(&format!("{}:", pattern_match.file.magenta()));
    }

    //show line number in green
    if show_number {
        output.push_str(&format!(
            "{}:",
            pattern_match.line_number.to_string().green()
        ));
    }

    // line with the matched pattern in bold/red
    let line = &pattern_match.content;
    let before = &line[..pattern_match.start];
    let pattern = &line[pattern_match.start..pattern_match.end];
    let after = &line[pattern_match.end..];

    output.push_str(&format!("{}{}{}", before, pattern.red().bold(), after));

    println!("{}", output);
}

// show the number of matches per file
pub fn show_count(file_name: &str, count: usize) {
    println!("{}:{}", file_name.magenta(), count.to_string().green());
}
