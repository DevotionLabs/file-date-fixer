mod cli;
mod logger;

use chrono::{DateTime, Local, NaiveDate, ParseResult, Utc};
use clap::Parser;
use cli::Cli;
use logger::{debug, error, fatal, info, warn};
use regex::Regex;
use std::fs;
use std::fs::DirEntry;
use std::io::Error;
use std::path::Path;

fn main() {
    let args = Cli::parse();
    let dir_path = Path::new(&args.dir);

    if !dir_path.is_dir() {
        fatal(format!("{} is not a directory", args.dir).as_str());
        std::process::exit(1);
    }

    info("Starting file date normalization process");

    process_dir(dir_path, args.recursive);
}

fn process_dir(dir: &Path, recursive: bool) {
    let files = fs::read_dir(dir).unwrap_or_else(|err| {
        error(&format!(
            "Error: Unable to read specified directory: {}",
            err
        ));
        std::process::exit(1);
    });

    for entry in files {
        process_dir_entry(entry, recursive);
    }
}

fn process_dir_entry(entry: Result<DirEntry, Error>, recursive: bool) {
    if let Ok(entry) = entry {
        let file_path = entry.path();

        if file_path.is_dir() {
            handle_directory_entry(&file_path, recursive);
        } else {
            handle_file_entry(&file_path);
        }
    }
}

fn handle_directory_entry(dir_path: &Path, recursive: bool) {
    if recursive {
        process_dir(dir_path, recursive);
    } else {
        warn(&format!(
            "Skipping directory: {}. Use -r to process directories recursively.",
            dir_path.display()
        ));
    }
}

fn handle_file_entry(file_path: &Path) {
    if let Some(file_name) = extract_utf8_file_name(file_path) {
        process_file(file_path, file_name);
    } else {
        error(&format!(
            "Unable to extract file name from path: {}",
            file_path.display()
        ));
    }
}

fn extract_utf8_file_name(path: &Path) -> Option<&str> {
    path.file_name()?.to_str()
}

fn process_file(file_path: &Path, file_name: &str) {
    let normalized_file_name = normalize_file_name(file_name);

    debug(&format!("Processing file: {}", file_name));

    if is_normalized_date_pattern_match(file_name) {
        process_matched_pattern_file(file_path, file_name, &normalized_file_name);
    }
}

/**
 * Normalize file name by replacing hyphens with underscores
 */
fn normalize_file_name(file_name: &str) -> String {
    file_name.replace("-", "_")
}

fn is_normalized_date_pattern_match(file_name: &str) -> bool {
    let pattern = Regex::new(r"^(IMG|VID|PANO)_\d{8}_.*").unwrap();
    pattern.is_match(file_name)
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

fn get_file_creation_date(path: &Path) -> Option<NaiveDate> {
    let metadata = fs::metadata(path).ok()?;
    let system_time = metadata.created().or_else(|_| metadata.modified()).ok()?;
    Some(DateTime::<Utc>::from(system_time).date_naive())
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

#[cfg(target_os = "windows")]
fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    use std::os::windows::fs::MetadataExt;
    use std::os::windows::fs::OpenOptionsExt;
    use std::ptr;
    use winapi::um::fileapi::SetFileTime;
    use winapi::um::minwinbase::FILETIME;
    use winapi::um::sysinfoapi::SystemTimeToFileTime;
    use winapi::um::timezoneapi::SystemTime;

    let file = OpenOptions::new().write(true).open(path)?;
    let mut sys_time = SystemTime {
        wYear: new_date.year() as u16,
        wMonth: new_date.month() as u16,
        wDay: new_date.day() as u16,
        ..Default::default()
    };
    let mut file_time: FILETIME = unsafe { std::mem::zeroed() };
    unsafe {
        SystemTimeToFileTime(&sys_time, &mut file_time);
        SetFileTime(file.as_raw_handle(), &file_time, ptr::null(), ptr::null());
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    use filetime::{set_file_times, FileTime};
    let new_file_time = FileTime::from_unix_time(
        new_date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp(),
        0,
    );
    set_file_times(path, new_file_time, new_file_time)?;
    Ok(())
}
