use ::regex::Regex;
use std::fs;
use std::io;
use std::path::Path;
use walkdir::WalkDir;

pub mod regex;
pub mod utils;

pub use regex::compile_regex;
pub use utils::SearchLine;

#[derive(Debug, PartialEq)]
pub struct SearchResult {
    pub file_name: String,
    pub lines: Vec<SearchLine>,
}

// searches the pattern on a single file and returns matches
pub fn search_file(
    regex: &Regex,
    file_path: &Path,
    invert_match: bool,
    before_context: Option<usize>,
    after_context: Option<usize>,
    context: Option<usize>,
) -> io::Result<SearchResult> {
    let file_content = fs::read_to_string(file_path)?;
    let file_name = file_path.display().to_string();

    let after = after_context.unwrap_or(0).max(context.unwrap_or(0));
    let before = before_context.unwrap_or(0).max(context.unwrap_or(0));

    let results = utils::search_content(&file_content, regex, invert_match, before, after);

    Ok(SearchResult {
        file_name,
        lines: results,
    })
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
