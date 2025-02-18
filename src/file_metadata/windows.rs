#[cfg(target_os = "windows")]
mod windows_only {
    use chrono::NaiveDate;
    use filetime::FileTime;
    use filetime_creation::set_file_ctime;
    use std::fs::Metadata;
    use std::io::Error;
    use std::path::Path;

    use super::super::common::get_file_date;

    pub fn get_file_creation_date(path: &Path) -> Option<NaiveDate> {
        get_file_date(path, Metadata::created)
    }

    #[cfg(target_os = "windows")]
    pub fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
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
}

#[cfg(target_os = "windows")]
pub use windows_only::*;
