use chrono::Local;
use std::path::PathBuf;

/// Get string of time with day.
/// {year}{month}{day}.
pub fn get_day_time() -> String {
    Local::now().format("%Y%m%d").to_string()
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
        .replace("{TIME_DAY}", &get_day_time())
        .replace("{TIME_SEC}", &get_sec_time())
        .replace("{TIME_MS}", &get_ms_time());
    PathBuf::from(&formatted_string)
}

#[test]
fn test_path() {
    println!("{}", get_day_time());
    println!("{}", get_sec_time());
    println!("{}", get_ms_time());
    println!("{:?}", format_time_macro("test_{TIME_DAY}".to_string()));
    println!("{:?}", format_time_macro("test_{TIME_SEC}".to_string()));
    println!("{:?}", format_time_macro("test_{TIME_MS}".to_string()));
}
