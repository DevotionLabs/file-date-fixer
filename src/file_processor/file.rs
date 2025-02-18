use std::path::Path;

use chrono::NaiveDate;

use super::{
    date::{extract_date_from_normalized_file_name, is_future_date, parse_normalized_date},
    normalization::{is_normalized_date_pattern_match, normalize_file_name},
};

use crate::logger::{debug, error, info, warn};

#[cfg(target_os = "linux")]
use crate::file_metadata::linux::{
    get_file_modification_date as get_file_metadata_date,
    set_file_modification_date as set_file_metadata_date,
};

#[cfg(target_os = "windows")]
use crate::file_metadata::windows::{
    get_file_creation_date as get_file_metadata_date,
    set_file_creation_date as set_file_metadata_date,
};

pub fn process_file(file_path: &Path, file_name: &str) {
    let normalized_file_name = normalize_file_name(file_name);

    debug(&format!("Processing file: {}", file_name));

    if is_normalized_date_pattern_match(normalized_file_name.as_str()) {
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

fn process_file_dates(file_path: &Path, file_name: &str, parsed_date: NaiveDate) {
    let file_date = match get_file_metadata_date(file_path) {
        Some(date) => date,
        None => {
            warn(&format!(
                "Unable to obtain file metadata date for file: {}",
                file_name
            ));
            return;
        }
    };

    debug(&format!("Obtained file metadata date: {}", file_date));

    if file_date > parsed_date {
        debug(&format!(
            "Updating file {}. File metadata date ({}) is newer than filename date ({})",
            file_name, file_date, parsed_date
        ));

        update_file_creation_date(file_path, file_name, parsed_date);
    }
}

fn update_file_creation_date(file_path: &Path, file_name: &str, new_creation_date: NaiveDate) {
    match set_file_metadata_date(file_path, new_creation_date) {
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
