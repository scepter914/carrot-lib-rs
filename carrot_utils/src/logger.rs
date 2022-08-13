use crate::path::format_time_macro;

use log::info;
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::fs;
use std::path::PathBuf;

/// Logger class
pub struct Logger {
    /// The root result of directory
    pub log_directory_path: PathBuf,
    pub log_file_path: PathBuf,
}

impl Logger {
    /// Init for Logger
    ///
    /// # Arguments
    /// - log_directory_path: The log directory path
    /// - log_file_name: The log file name
    /// - log_file_level: The log level for file logger
    /// - log_terminal_level: The log level for terminal logger
    pub fn new(
        log_directory_path: impl Into<String>,
        log_file_name: impl Into<String>,
        log_file_level: LevelFilter,
        log_terminal_level: LevelFilter,
    ) -> Logger {
        let formatted_log_directory_path: PathBuf = format_time_macro(log_directory_path.into());
        let formatted_log_file_path: PathBuf =
            formatted_log_directory_path.join(format_time_macro(log_file_name.into()));
        let _ = fs::create_dir_all(&formatted_log_directory_path);

        // logger init
        CombinedLogger::init(vec![
            TermLogger::new(
                log_terminal_level,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                log_file_level,
                Config::default(),
                fs::File::create(&formatted_log_file_path).unwrap(),
            ),
        ])
        .unwrap();
        info!("Make log file to {:?}", &formatted_log_directory_path);

        Logger {
            log_directory_path: formatted_log_directory_path,
            log_file_path: formatted_log_file_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let _ = Logger::new(
            "./data/{TIME_SEC}",
            "log_{TIME_SEC}.txt",
            LevelFilter::Debug,
            LevelFilter::Debug,
        );
    }
}
