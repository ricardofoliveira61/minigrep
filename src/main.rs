mod cli;
mod formatter;
mod search;

use clap::Parser;
use std::process;

fn main() {
    let args = cli::Arguments::parse();

    //compile regex
    let pattern_regex = match search::compile_regex(&args.pattern, args.ignore_case) {
        Ok(r) => r,
        Err(e) => {
            eprint!("Error on the regex pattern '{}':{}", args.pattern, e);
            process::exit(1);
        }
    };

    let file_paths = search::collect_files(&args.paths, args.recursive);

    if file_paths.is_empty() {
        eprint!("minigrep: no file found to match pattern.");
        process::exit(1);
    }

    let show_file_name = file_paths.len() > 1;
    let mut error_found = false;

    for file_path in &file_paths {
        let file_path = std::path::Path::new(file_path);

        match search::search_file(&pattern_regex, file_path) {
            Ok(results) => {
                if args.count {
                    formatter::show_count(&file_path.display().to_string(), results.len());
                } else {
                    for result in &results {
                        formatter::show_match(result, show_file_name, args.line_number);
                    }
                }
            }
            Err(e) => {
                eprint!("minigrep: error reading '{}':{}", file_path.display(), e);
                error_found = true;
            }
        }
    }

    if error_found {
        process::exit(2);
    }
}
