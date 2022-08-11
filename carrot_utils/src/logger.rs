use crate::path;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode};
use std::fs;
use std::path::PathBuf;

/// Logger class
pub struct Logger {
    /// The root result of directory
    pub root_result_directory: PathBuf,
}

impl Logger {
    /// Init for CarrotLogger
    /// # Arguments
    /// - log_directory_path: The log directory path
    /// - log_file_level: The log level for file logger
    /// - log_terninal_level: The log level for terninal logger
    pub fn init(
        log_directory_path: impl Into<PathBuf>,
        log_file_level: LevelFilter,
        log_terninal_level: LevelFilter,
    ) -> Logger {
        let root_result_directory = log_directory_path
            .into()
            .join(path::get_time_str_with_sec());
        let _ = fs::create_dir(&root_result_directory);
        let log_path = root_result_directory.join(path::get_time_filepath_sec("log_", "txt"));

        // logger init
        CombinedLogger::init(vec![
            TermLogger::new(
                log_terninal_level,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                log_file_level,
                Config::default(),
                fs::File::create(log_path).unwrap(),
            ),
        ])
        .unwrap();

        Logger {
            root_result_directory,
        }
    }
}
