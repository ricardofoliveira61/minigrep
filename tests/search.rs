use minigrep::search::{SearchLine, SearchResult, collect_files, search_file};
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

#[test]
fn collect_files_should_include_direct_file_paths_regardless_of_recursive_flag() {
    let temp_file = NamedTempFile::new().unwrap();
    let path_str = temp_file.path().display().to_string();
    let paths = vec![path_str.clone()];

    let result_non_recursive = collect_files(&paths, false);
    let result_recursive = collect_files(&paths, true);

    assert_eq!(result_non_recursive, vec![path_str.clone()]);
    assert_eq!(result_recursive, vec![path_str]);
}

#[test]
fn collect_files_should_recursively_discover_all_nested_files_when_recursive_is_true() {
    let temp_dir = TempDir::new().unwrap();

    let nested_dir = temp_dir.path().join("nested_dir");
    fs::create_dir(&nested_dir).unwrap();

    let top_file_path = temp_dir.path().join("top_level.txt");
    let deep_file_path = nested_dir.join("deep_level.txt");

    File::create(&top_file_path).unwrap();
    File::create(&deep_file_path).unwrap();

    let paths = vec![temp_dir.path().display().to_string()];

    let mut results = collect_files(&paths, true);
    results.sort();

    let mut expected = vec![
        top_file_path.display().to_string(),
        deep_file_path.display().to_string(),
    ];
    expected.sort();

    assert_eq!(results, expected);
}

#[test]
fn collect_files_should_ignore_directory_contents_when_recursive_is_false() {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("ignored.txt");
    File::create(&file_path).unwrap();

    let paths = vec![temp_dir.path().display().to_string()];

    let results = collect_files(&paths, false);

    assert!(
        results.is_empty(),
        "Expected no files to be collected from a directory when recursive is false"
    );
}

#[test]
fn search_file_should_read_file_and_extract_matches_with_correct_metadata() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "apple\nbanana\ncherry").unwrap();

    let regex = Regex::new("banana").unwrap();
    let path = temp_file.path();

    let result = search_file(&regex, path, false, None, None, None).unwrap();

    let expected = SearchResult {
        file_name: path.display().to_string(),
        lines: vec![SearchLine::Match {
            line_number: 2,
            content: "banana".to_string(),
            start: 0,
            end: 6,
        }],
    };

    assert_eq!(result, expected);
}

#[test]
fn search_file_should_use_the_maximum_value_between_explicit_context_and_general_context() {
    let mut temp_file = NamedTempFile::new().unwrap();
    writeln!(temp_file, "line1\nline2\nmatch\nline4\nline5").unwrap();

    let regex = Regex::new("match").unwrap();
    let path = temp_file.path();

    // context=2 dominates before_context=None and after_context=Some(1)
    let result = search_file(&regex, path, false, None, Some(1), Some(2)).unwrap();

    assert_eq!(
        result.lines.len(),
        5,
        "Expected all 5 lines to be returned as context/matches"
    );
    assert!(matches!(
        result.lines[0],
        SearchLine::ContextLine { line_number: 1, .. }
    ));
    assert!(matches!(
        result.lines[4],
        SearchLine::ContextLine { line_number: 5, .. }
    ));
}

#[test]
fn search_file_should_bubble_up_io_error_when_file_does_not_exist() {
    let regex = Regex::new("dummy").unwrap();
    let non_existent_path = Path::new("this_file_does_not_exist_anywhere.txt");

    let result = search_file(&regex, non_existent_path, false, None, None, None);

    assert!(
        result.is_err(),
        "Expected an io::Error when reading a missing file"
    );
    assert_eq!(result.unwrap_err().kind(), std::io::ErrorKind::NotFound);
}
