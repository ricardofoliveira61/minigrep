use regex::Regex;

// compiles the regext pattern
pub fn compile_regex(pattern: &str, ignore_case: bool) -> Result<Regex, regex::Error> {
    let final_pattern = if ignore_case {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };

    Regex::new(&final_pattern)
}
