use crate::path::format_time_macro;

use log::{info, LevelFilter, SetLoggerError};
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::fs;
use std::path::PathBuf;

/// Get log level from &str
pub fn get_log_level(log_level: &str) -> LevelFilter {
    match &log_level[..] {
        "Error" => LevelFilter::Error,
        "Warn" => LevelFilter::Warn,
        "Info" => LevelFilter::Info,
        "Debug" => LevelFilter::Debug,
        "Trace" => LevelFilter::Trace,
        _ => LevelFilter::Info,
    }
}

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
    /// - log_file_enable_level: The enable level for terminal logger
    pub fn new(
        log_directory_path: impl Into<String>,
        log_file_name: impl Into<String>,
        log_file_level: LevelFilter,
        log_terminal_level: LevelFilter,
        log_file_enable_level: LevelFilter,
    ) -> Logger {
        let formatted_log_directory_path: PathBuf = format_time_macro(log_directory_path.into());
        let formatted_log_file_path: PathBuf =
            formatted_log_directory_path.join(format_time_macro(log_file_name.into()));

        if log_file_level >= log_file_enable_level {
            let _ = Self::init_logger_with_log_files(
                &formatted_log_directory_path,
                &formatted_log_file_path,
                log_file_level,
                log_terminal_level,
            );
        } else {
            let _ = Self::init_logger_without_log_files(log_terminal_level);
        }

        Logger {
            log_directory_path: formatted_log_directory_path,
            log_file_path: formatted_log_file_path,
        }
    }

    fn init_logger_with_log_files(
        log_directory_path: &PathBuf,
        log_file_path: &PathBuf,
        log_file_level: LevelFilter,
        log_terminal_level: LevelFilter,
    ) -> Result<(), SetLoggerError> {
        let _ = fs::create_dir_all(log_directory_path);
        info!("Make log file to {:?}", log_directory_path);

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
                fs::File::create(log_file_path).unwrap(),
            ),
        ])
        //.unwrap();
    }

    fn init_logger_without_log_files(
        log_terminal_level: LevelFilter,
    ) -> Result<(), SetLoggerError> {
        // logger init
        TermLogger::init(
            log_terminal_level,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::debug;

    #[test]
    fn test_logger() {
        let _ = Logger::new(
            "./data/{TIME_SEC}",
            "log_{TIME_SEC}.txt",
            LevelFilter::Debug,
            LevelFilter::Debug,
            LevelFilter::Debug,
        );
        info!("[info] Logger with log file. this message can be printed.");
        debug!("[debug] Logger with log file. this message can be printed.");

        let _ = Logger::new(
            "./data/{TIME_SEC}",
            "log_{TIME_SEC}.txt",
            LevelFilter::Info,
            LevelFilter::Info,
            LevelFilter::Debug,
        );
        info!("[info] Logger without log file. this message can be printed.");
        debug!("[debug] Logger without log file. this message cannot be printed.");
    }
}
