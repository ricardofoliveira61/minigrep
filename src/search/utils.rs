use ::regex::Regex;

#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use ::regex::Regex;

    const TOTAL_LINES: usize = 6;

    #[test]
    fn calculate_merged_ranges_should_return_individual_ranges_when_no_context_and_no_overlap() {
        let match_indices = vec![1, 3, 5];
        let after = 0;
        let before = 0;

        let expected_res = vec![(1, 2), (3, 4), (5, 6)];

        assert_eq!(
            calculate_merged_ranges(&match_indices, TOTAL_LINES, before, after),
            expected_res
        );
    }

    #[test]
    fn calculate_merged_ranges_should_merge_adjacent_indices_when_no_context() {
        let match_indices = vec![1, 2, 5];
        let after = 0;
        let before = 0;

        let expected_res = vec![(1, 3), (5, 6)];

        assert_eq!(
            calculate_merged_ranges(&match_indices, TOTAL_LINES, before, after),
            expected_res
        );
    }

    #[test]
    fn calculate_merged_ranges_should_expand_ranges_by_context_without_merging() {
        let match_indices = vec![1, 5];
        let after = 1;
        let before = 1;

        let expected_res = vec![(0, 3), (4, 6)];

        assert_eq!(
            calculate_merged_ranges(&match_indices, TOTAL_LINES, before, after),
            expected_res
        );
    }

    #[test]
    fn calculate_merged_ranges_should_merge_into_single_range_when_context_overlaps() {
        let match_indices = vec![1, 4];
        let after = 1;
        let before = 1;

        let expected_res = vec![(0, 6)];

        assert_eq!(
            calculate_merged_ranges(&match_indices, TOTAL_LINES, before, after),
            expected_res
        );
    }

    #[test]
    fn calculate_merged_ranges_no_match_index() {
        let match_indices = vec![];
        let after = 1;
        let before = 1;

        let expected_res = vec![];

        assert_eq!(
            calculate_merged_ranges(&match_indices, TOTAL_LINES, before, after),
            expected_res
        );
    }

    #[test]
    fn search_content_should_return_exact_match_bounds_when_no_context_requested() {
        let content = "apple\nbanana\ncherry";
        let regex = Regex::new("anana").unwrap();

        let result = search_content(content, &regex, false, 0, 0);

        let expected = vec![SearchLine::Match {
            line_number: 2,
            content: "banana".to_string(),
            start: 1,
            end: 6,
        }];

        assert_eq!(result, expected);
    }

    #[test]
    fn search_content_should_include_context_lines_when_before_and_after_are_set() {
        let content = "line1\nline2\nmatch_here\nline4\nline5";
        let regex = Regex::new("match").unwrap();

        // Request 1 line before and 1 line after
        let result = search_content(content, &regex, false, 1, 1);

        let expected = vec![
            SearchLine::ContextLine {
                line_number: 2,
                content: "line2".to_string(),
            },
            SearchLine::Match {
                line_number: 3,
                content: "match_here".to_string(),
                start: 0,
                end: 5,
            },
            SearchLine::ContextLine {
                line_number: 4,
                content: "line4".to_string(),
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn search_content_should_invert_matches_when_invert_match_is_true() {
        let content = "apple\nbanana";
        let regex = Regex::new("apple").unwrap();

        // Look for lines that DO NOT match "apple"
        let result = search_content(content, &regex, true, 0, 0);

        let expected = vec![SearchLine::Match {
            line_number: 2,
            content: "banana".to_string(),
            start: 0,
            end: 6,
        }];

        assert_eq!(result, expected);
    }

    #[test]
    fn search_content_should_handle_overlapping_contexts_gracefully() {
        let content = "match1\ncontext\nmatch2";
        let regex = Regex::new("match").unwrap();

        // Context overlap! Both matches want line 2 as context.
        let result = search_content(content, &regex, false, 1, 1);

        let expected = vec![
            SearchLine::Match {
                line_number: 1,
                content: "match1".to_string(),
                start: 0,
                end: 5,
            },
            SearchLine::ContextLine {
                line_number: 2,
                content: "context".to_string(),
            },
            SearchLine::Match {
                line_number: 3,
                content: "match2".to_string(),
                start: 0,
                end: 5,
            },
        ];

        assert_eq!(result, expected);
    }
}
