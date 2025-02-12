use regex::Regex;

/**
 * Normalize file name by replacing hyphens with underscores
 */
pub fn normalize_file_name(file_name: &str) -> String {
    file_name.replace("-", "_")
}

pub fn is_normalized_date_pattern_match(file_name: &str) -> bool {
    let pattern = Regex::new(r"^(IMG|VID|PANO)_\d{8}_.*").unwrap();
    pattern.is_match(file_name)
}
