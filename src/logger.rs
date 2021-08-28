extern crate chrono;
extern crate simplelog;

use chrono::Local;
use simplelog::Level;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct Logger {
    logger_directory_path: PathBuf,
    logger_level: Level,
}

impl Logger {
    pub fn init(log_level: &str, directory_path: impl Into<PathBuf>) -> Logger {
        let logger_level_ = set_log_level(log_level);
        let directory_path_with_time = directory_path
            .into()
            .join(get_now_time_without_millisecond());
        let _ = fs::create_dir(&directory_path_with_time);
        let logger = Logger {
            logger_directory_path: directory_path_with_time.to_path_buf(),
            logger_level: logger_level_,
        };
        logger.logger_init();
        return logger;
    }

    fn logger_init(&self) {
        simplelog::CombinedLogger::init(vec![
            simplelog::TermLogger::new(
                self.logger_level.to_level_filter(),
                simplelog::Config::default(),
                simplelog::TerminalMode::Mixed,
                simplelog::ColorChoice::Auto,
            ),
            simplelog::WriteLogger::new(
                self.logger_level.to_level_filter(),
                simplelog::Config::default(),
                fs::File::create(self.get_time_path_without_millisecond("log_", "txt")).unwrap(),
            ),
        ])
        .unwrap();
    }

    pub fn get_full_path(&self, file_path: impl Into<PathBuf>) -> PathBuf {
        self.logger_directory_path.join(file_path.into())
    }

    pub fn get_time_path(
        &self,
        file_path: impl Into<PathBuf>,
        extension: impl Into<PathBuf>,
    ) -> PathBuf {
        self.logger_directory_path
            .join(file_path.into())
            .join(Path::new(&get_now_time()).with_extension(extension.into()))
    }

    pub fn get_time_path_without_millisecond(
        &self,
        file_path: impl Into<String>,
        extension: impl Into<String>,
    ) -> PathBuf {
        self.logger_directory_path
            .join(file_path.into())
            .join(Path::new(&get_now_time_without_millisecond()).with_extension(extension.into()))
    }
}

fn set_log_level(log_level: &str) -> Level {
    let logger_level: Level;
    match &log_level[..] {
        "Error" => logger_level = Level::Error,
        "Warn" => logger_level = Level::Warn,
        "Info" => logger_level = Level::Info,
        "Debug" => logger_level = Level::Debug,
        "Trace" => logger_level = Level::Trace,
        _ => logger_level = Level::Error,
    }
    return logger_level;
}

fn get_now_time() -> String {
    return Local::now().format("%Y%m%d_%H%M_%S_%3f").to_string();
}

fn get_now_time_without_millisecond() -> String {
    return Local::now().format("%Y%m%d_%H%M").to_string();
}
