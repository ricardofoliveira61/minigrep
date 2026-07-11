use regex::Regex;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

// Represents a pattern match inside a file
#[derive(Debug)]
pub struct Match {
    pub file: String,
    pub line_number: usize,
    pub content: String,
    pub start: usize,
    pub end: usize,
}

// compiles the regext pattern
pub fn compile_regex(pattern: &str, ignore_case: bool) -> Result<Regex, regex::Error> {
    let final_pattern = if ignore_case {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };

    Regex::new(&final_pattern)
}

// searches the pattern on a single file and returns matches
pub fn search_file(regex: &Regex, file_path: &Path) -> io::Result<Vec<Match>> {
    let file_content = fs::read_to_string(file_path)?;
    let file_name = file_path.display().to_string();

    let mut results = Vec::new();

    for (index, line_content) in file_content.lines().enumerate() {
        if let Some(pattern_match) = regex.find(line_content) {
            results.push(Match {
                file: file_name.clone(),
                line_number: index + 1,
                content: line_content.to_string(),
                start: pattern_match.start(),
                end: pattern_match.end(),
            })
        }
    }

    Ok(results)
}

pub fn collect_files(paths: &[String], recursive: bool) -> Vec<String> {
    let mut files_path = Vec::new();

    for path in paths {
        let path = Path::new(path);

        if path.is_file() {
            files_path.push(path.display().to_string());
        } else if path.is_dir() && recursive {
            for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
                if entry.file_type().is_file() {
                    files_path.push(entry.path().display().to_string())
                }
            }
        } else if path.is_dir() {
            eprint!(
                "minigrep: {} its a directory. Use -r for recursive search",
                path.display()
            )
        }
    }

    files_path
}
