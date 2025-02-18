use chrono::NaiveDate;
use filetime::{set_file_mtime, FileTime};
use std::fs::Metadata;
use std::io::Error;
use std::path::Path;

use super::common::get_file_date;

pub fn get_file_modification_date(path: &Path) -> Option<NaiveDate> {
    get_file_date(path, Metadata::modified)
}

#[cfg(target_os = "linux")]
pub fn set_file_modification_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    let new_file_time = FileTime::from_unix_time(
        new_date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp(),
        0,
    );

    set_file_mtime(path, new_file_time)?;
    Ok(())
}
