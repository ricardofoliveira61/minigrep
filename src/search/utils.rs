use ::regex::Regex;

#[derive(Debug)]
pub enum SearchLine {
    ContextLine {
        line_number: usize,
        content: String,
    },
    Match {
        line_number: usize,
        content: String,
        start: usize,
        end: usize,
    },
}

impl SearchLine {
    pub fn get_line_number(&self) -> usize {
        match self {
            SearchLine::ContextLine { line_number, .. } => *line_number,
            SearchLine::Match { line_number, .. } => *line_number,
        }
    }
}

pub fn search_content(
    content: &str,
    regex: &Regex,
    invert_match: bool,
    before: usize,
    after: usize,
) -> Vec<SearchLine> {
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    let mut matches: Vec<(usize, (usize, usize))> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let regex_match = regex.find(line);
        let is_match = regex_match.is_some();

        if invert_match != is_match {
            let bounds = match regex_match {
                Some(m) if !invert_match => (m.start(), m.end()),
                _ => (0, line.len()),
            };

            matches.push((index, bounds));
        }
    }

    let match_indices: Vec<usize> = matches.iter().map(|(idx, _)| *idx).collect();
    let merged_ranges = calculate_merged_ranges(&match_indices, total_lines, before, after);

    let mut results = Vec::new();
    let mut matches_iter = matches.into_iter().peekable();

    for (start, end) in merged_ranges {
        for (local_index, line) in lines[start..end].iter().enumerate() {
            let global_index = start + local_index;

            if matches_iter
                .peek()
                .is_some_and(|(match_idx, _)| *match_idx == global_index)
            {
                let (_, (s, e)) = matches_iter.next().unwrap();
                results.push(SearchLine::Match {
                    line_number: global_index + 1,
                    content: line.to_string(),
                    start: s,
                    end: e,
                });
            } else {
                results.push(SearchLine::ContextLine {
                    line_number: global_index + 1,
                    content: line.to_string(),
                });
            }
        }
    }

    results
}

fn calculate_merged_ranges(
    match_indices: &[usize],
    total_lines: usize,
    before: usize,
    after: usize,
) -> Vec<(usize, usize)> {
    let mut merged_ranges: Vec<(usize, usize)> = Vec::new();

    for m in match_indices {
        let start = m.saturating_sub(before);
        let end = (m + after + 1).min(total_lines);

        if let Some(last_range) = merged_ranges.last_mut() {
            if start <= last_range.1 {
                last_range.1 = last_range.1.max(end)
            } else {
                merged_ranges.push((start, end));
            }
        } else {
            merged_ranges.push((start, end));
        }
    }

    merged_ranges
}
