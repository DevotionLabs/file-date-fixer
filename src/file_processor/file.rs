use std::path::Path;

use chrono::{Local, NaiveDate, ParseResult};

use super::normalization::{is_normalized_date_pattern_match, normalize_file_name};

use crate::{
    file_metadata::{get_file_creation_date, set_file_creation_date},
    logger::{debug, error, info, warn},
};

pub fn process_file(file_path: &Path, file_name: &str) {
    let normalized_file_name = normalize_file_name(file_name);

    debug(&format!("Processing file: {}", file_name));

    if is_normalized_date_pattern_match(file_name) {
        process_matched_pattern_file(file_path, file_name, &normalized_file_name);
    }
}

fn process_matched_pattern_file(file_path: &Path, file_name: &str, normalized_file_name: &str) {
    let date_str = extract_date_from_normalized_file_name(normalized_file_name);

    match parse_normalized_date(date_str) {
        Ok(parsed_date) => handle_valid_parsed_date(file_path, file_name, parsed_date),
        Err(_) => {
            error(&format!(
                "Skipping file {}. It does not contain a valid date in its name",
                file_name
            ));
        }
    }
}

fn extract_date_from_normalized_file_name(file_name: &str) -> &str {
    file_name.split('_').nth(1).unwrap()
}

fn parse_normalized_date(date_str: &str) -> ParseResult<NaiveDate> {
    NaiveDate::parse_from_str(date_str, "%Y%m%d")
}

fn handle_valid_parsed_date(file_path: &Path, file_name: &str, parsed_date: NaiveDate) {
    debug(&format!("Parsed date: {}", parsed_date));

    if is_future_date(parsed_date) {
        warn(&format!(
            "Skipping file {}. It contains a future date",
            file_name
        ));
        return;
    }

    process_file_dates(file_path, file_name, parsed_date);
}

fn is_future_date(parsed_date: NaiveDate) -> bool {
    parsed_date > Local::now().date_naive()
}

fn process_file_dates(file_path: &Path, file_name: &str, parsed_date: NaiveDate) {
    let file_creation_date = match get_file_creation_date(file_path) {
        Some(date) => date,
        None => {
            warn(&format!(
                "Unable to obtain file creation date for file: {}",
                file_name
            ));
            return;
        }
    };

    debug(&format!(
        "Obtained file creation date: {}",
        file_creation_date
    ));

    if file_creation_date > parsed_date {
        debug(&format!(
            "Updating file {}. Creation date ({}) is newer than filename date ({})",
            file_name, file_creation_date, parsed_date
        ));

        update_file_creation_date(file_path, file_name, parsed_date);
    }
}

fn update_file_creation_date(file_path: &Path, file_name: &str, new_creation_date: NaiveDate) {
    match set_file_creation_date(file_path, new_creation_date) {
        Ok(_) => info(&format!(
            "Successfully updated file creation date for file: {}",
            file_name
        )),

        Err(e) => error(&format!(
            "Failed to update creation date for file {}: {}",
            file_name, e
        )),
    }
}
