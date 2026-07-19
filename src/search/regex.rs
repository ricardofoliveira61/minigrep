use regex::Regex;

pub fn compile_regex(pattern: &str, ignore_case: bool) -> Result<Regex, regex::Error> {
    let final_pattern = if ignore_case {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };

    Regex::new(&final_pattern)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_regex_should_succeed_when_given_valid_literal_pattern() {
        let pattern = "apple";
        let result = compile_regex(pattern, false);

        assert!(
            result.is_ok(),
            "Expected a valid pattern to compile successfully"
        );

        let regex = result.unwrap();
        assert!(regex.is_match("an apple a day"));
        assert!(!regex.is_match("banana"));
    }

    #[test]
    fn compile_regex_should_succeed_when_given_valid_complex_expression() {
        // A pattern for finding digits
        let pattern = r"\d+";
        let result = compile_regex(pattern, false);

        assert!(result.is_ok());

        let regex = result.unwrap();
        assert!(regex.is_match("Room 404"));
        assert!(!regex.is_match("No numbers here"));
    }

    #[test]
    fn compile_regex_should_return_error_when_given_invalid_syntax() {
        let invalid_pattern = "match_this_(";
        let result = compile_regex(invalid_pattern, false);

        assert!(
            result.is_err(),
            "Expected an invalid regex syntax to return an Error"
        );
    }

    #[test]
    fn compile_regex_case_sensitive_does_not_match_uppercase() {
        let regex = compile_regex("hello", false).unwrap();

        assert!(!regex.is_match("HELLO"));
        assert!(regex.is_match("hello"));
    }

    #[test]
    fn compile_regex_should_match_regardless_of_case_when_case_insensitive_is_true() {
        let pattern = "apple";
        let result = compile_regex(pattern, true);

        assert!(result.is_ok());
        let regex = result.unwrap();

        assert!(regex.is_match("APPLE"), "Should match uppercase text");
        assert!(regex.is_match("aPpLe"), "Should match mixed-case text");
        assert!(
            regex.is_match("apple"),
            "Should still match exact lowercase text"
        );
    }
}
