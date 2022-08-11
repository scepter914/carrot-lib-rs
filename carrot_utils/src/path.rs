use chrono::Local;
use std::path::PathBuf;

/// Get string of time with day.
/// {year}{month}{day}.
pub fn get_day_time() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Get string of time with second.
/// {year}{month}{day}-{hour}{minutes}{second}.
pub fn get_sec_time() -> String {
    Local::now().format("%Y%m%d_%H%M%S").to_string()
}

/// Get string of time with millisecond.
/// {year}{month}{day}-{hour}{minutes}{second}-{millisecond}.
pub fn get_ms_time() -> String {
    Local::now().format("%Y%m%d_%H%M%S_%3f").to_string()
}

/// Format time micro
/// "TIME_aaa_bbb" ->  "20220101_010203_aaa_bbb"
pub fn format_time_macro(string: String) -> PathBuf {
    let formatted_string: String = string
        .replace("TIME_DAY", &get_day_time())
        .replace("TIME_SEC", &get_sec_time())
        .replace("TIME_MS", &get_ms_time());
    PathBuf::from(&formatted_string)
}
