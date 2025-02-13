use chrono::{DateTime, NaiveDate, Utc};
use std::fs;
use std::io::Error;
use std::path::Path;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This program only supports Windows and Linux.");

pub fn get_file_creation_date(path: &Path) -> Option<NaiveDate> {
    let metadata = fs::metadata(path).ok()?;
    let system_time = metadata.created().or_else(|_| metadata.modified()).ok()?;
    Some(DateTime::<Utc>::from(system_time).date_naive())
}

#[cfg(target_os = "windows")]
pub fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    use filetime::FileTime;
    use filetime_creation::set_file_ctime;

    let new_date_time = new_date.and_hms_opt(0, 0, 0).ok_or_else(|| {
        Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid date. Could not convert date to datetime",
        )
    })?;

    let utc_date_time = new_date_time.and_utc();
    let unix_timestamp = utc_date_time.timestamp();
    let nanos = utc_date_time.timestamp_subsec_nanos();

    let filetime = FileTime::from_unix_time(unix_timestamp, nanos);

    set_file_ctime(path, filetime).map_err(|e| {
        Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to set file creation time for {:?}: {}", path, e),
        )
    })?;

    Ok(())
}

#[cfg(target_os = "linux")]
pub fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    use filetime::{set_file_times, FileTime};
    let new_file_time = FileTime::from_unix_time(
        new_date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp(),
        0,
    );
    set_file_times(path, new_file_time, new_file_time)?;
    Ok(())
}
