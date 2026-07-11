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
pub fn search_file(regex: &Regex, file_path: &Path, invert_match: bool) -> io::Result<Vec<Match>> {
    let file_content = fs::read_to_string(file_path)?;
    let file_name = file_path.display().to_string();

    let mut results = Vec::new();

    for (index, line_content) in file_content.lines().enumerate() {
        let result = regex.find(line_content);

        if invert_match == result.is_some() {
            continue;
        }

        let (start, end) = result
            .map(|m| (m.start(), m.end()))
            .unwrap_or((0, line_content.len()));

        results.push(Match {
            file: file_name.clone(),
            line_number: index + 1,
            content: line_content.to_string(),
            start,
            end,
        })
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
            eprintln!(
                "minigrep: {} its a directory. Use -r for recursive search",
                path.display()
            )
        }
    }

    files_path
}
