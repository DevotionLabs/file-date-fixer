use chrono::{DateTime, NaiveDate, Utc};
use std::fs;
use std::io::Error;
use std::path::Path;

#[cfg(target_os = "windows")]
use winapi::shared::minwindef::FILETIME;

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
compile_error!("This program only supports Windows and Linux.");

pub fn get_file_creation_date(path: &Path) -> Option<NaiveDate> {
    let metadata = fs::metadata(path).ok()?;
    let system_time = metadata.created().or_else(|_| metadata.modified()).ok()?;
    Some(DateTime::<Utc>::from(system_time).date_naive())
}

#[cfg(target_os = "windows")]
fn system_time_to_filetime(timestamp: i64) -> FILETIME {
    let windows_epoch = 11644473600i64; // Difference between UNIX and Windows epoch
    let timestamp_100ns = (timestamp + windows_epoch) * 10_000_000;

    FILETIME {
        dwLowDateTime: (timestamp_100ns & 0xFFFFFFFF) as u32,
        dwHighDateTime: (timestamp_100ns >> 32) as u32,
    }
}

#[cfg(target_os = "windows")]
pub fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    use std::fs::OpenOptions;
    use std::os::windows::io::AsRawHandle;
    use std::ptr::null_mut;
    use winapi::um::fileapi::SetFileTime;
    use winapi::um::winnt::HANDLE;

    let new_date_time = new_date.and_hms_opt(0, 0, 0).unwrap();
    let new_date_unix_timestamp = new_date_time.and_utc().timestamp();

    let file = OpenOptions::new().write(true).open(path)?;
    let handle = file.as_raw_handle() as HANDLE;

    // TODO: Manage invalid handle value

    let creation_time_ft = system_time_to_filetime(new_date_unix_timestamp);

    let success = unsafe { SetFileTime(handle, &creation_time_ft, null_mut(), null_mut()) };

    if success == 0 {
        return Err(Error::last_os_error());
    }

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
