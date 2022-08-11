use crate::path::format_time_macro;

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
    pub fn init(
        log_directory_path: String,
        log_file_name: String,
        log_file_level: LevelFilter,
        log_terminal_level: LevelFilter,
    ) -> Logger {
        let formatted_log_directory_path: PathBuf = format_time_macro(log_directory_path);
        let formatted_log_file_path: PathBuf =
            formatted_log_directory_path.join(format_time_macro(log_file_name));

        let _ = fs::create_dir(&formatted_log_directory_path);

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

        Logger {
            log_directory_path: formatted_log_directory_path,
            log_file_path: formatted_log_file_path,
        }
    }
}
