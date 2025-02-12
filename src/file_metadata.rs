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
pub fn set_file_creation_date(path: &Path, new_date: NaiveDate) -> Result<(), Error> {
    use filetime::{set_file_times, FileTime};
    let new_file_time = FileTime::from_unix_time(
        new_date.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp(),
        0,
    );
    set_file_times(path, new_file_time, new_file_time)?;
    Ok(())
}
