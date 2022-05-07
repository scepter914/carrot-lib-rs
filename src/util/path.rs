use chrono::Local;
use std::path::{Path, PathBuf};

/// Get string of time
/// {year}-{month}-{day}-{hour}-{minutes}
pub fn get_time_str() -> String {
    Local::now().format("%Y%m%d_%H%M").to_string()
}

/// Get string of time with second.
/// {year}-{month}-{day}-{hour}-{minutes}-{second}.
pub fn get_time_str_with_sec() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Get string of time with millisecond.
/// {year}-{month}-{day}-{hour}-{minutes}-{second}-{millisecond}
pub fn get_time_str_with_ms() -> String {
    Local::now().format("%Y%m%d_%H%M%S_%3f").to_string()
}

/// Get time file path
/// "{file_name}{now_time}.{extension}"
pub fn get_time_filepath(file_name: impl Into<String>, extension: impl Into<String>) -> PathBuf {
    let file_name_with_time = format!("{}{}", file_name.into(), get_time_str());
    let file_with_extension = Path::new(&file_name_with_time).with_extension(extension.into());
    file_with_extension
}

/// Get time file path with second.
/// "{file_name}{now_time}.{extension}"
pub fn get_time_filepath_sec(
    file_name: impl Into<String>,
    extension: impl Into<String>,
) -> PathBuf {
    let file_name_with_time = format!("{}{}", file_name.into(), get_time_str_with_sec());
    let file_with_extension = Path::new(&file_name_with_time).with_extension(extension.into());
    file_with_extension
}

/// Get time file path with millisecond.
/// "{file_name}{now_time}.{extension}"
pub fn get_time_filepath_ms(file_name: impl Into<String>, extension: impl Into<String>) -> PathBuf {
    let file_name_with_time = format!("{}{}", file_name.into(), get_time_str_with_ms());
    let file_with_extension = Path::new(&file_name_with_time).with_extension(extension.into());
    file_with_extension
}
