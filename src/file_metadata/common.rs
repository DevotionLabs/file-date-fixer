use chrono::{DateTime, NaiveDate, Utc};
use std::fs;
use std::path::Path;
use std::time::SystemTime;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This program only supports Windows and Linux.");

pub fn get_file_date(
    path: &Path,
    time_fn: fn(&fs::Metadata) -> std::io::Result<SystemTime>,
) -> Option<NaiveDate> {
    let metadata = fs::metadata(path).ok()?;
    let system_time = time_fn(&metadata).ok()?;
    Some(DateTime::<Utc>::from(system_time).date_naive())
}
