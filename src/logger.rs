use chrono::Local;
use log::{Level, LevelFilter, Log, Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

pub enum LogOutput {
    Stdout,
    Stderr,
}

pub struct Logger {
    pub write_to_file: bool,
    pub write_to_std: Option<LogOutput>,
    pub severity: Level,
    pub file: Option<Arc<Mutex<File>>>,
    pub enable_colors: bool,
}

impl Logger {
    pub fn new(
        file_path: Option<PathBuf>,
        severity: Option<Level>,
        write_to_std: Option<LogOutput>,
        write_to_file: bool,
        enable_colors: bool,
    ) -> Self {
        let mut path = file_path;
        if path.is_none() {
            path = Some(PathBuf::from(
                #[cfg(target_os = "linux")]
                "/var/log/nornity/nornity.log",
                #[cfg(target_os = "windows")]
                "C:\\Program Files\\nornity\\nornity.log",
                #[cfg(target_os = "macos")]
                "/Library/Logs/nornity/nornity.log",
                #[cfg(target_os = "freebsd")]
                "/var/log/nornity/nornity.log",
                #[cfg(not(any(
                    target_os = "linux",
                    target_os = "windows",
                    target_os = "macos",
                    target_os = "freebsd"
                )))]
                "/var/log/nornity/nornity.log",
            ));
        }
        let mut file = None;

        // Create log directory if it doesn't exist
        if let Some(path_ref) = path.as_ref() {
            if let Some(parent) = path_ref.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
        }

        if write_to_file {
            if let Some(path_ref) = path.as_ref() {
                file = File::create(path_ref).ok().map(|f| Arc::new(Mutex::new(f)));
            }
        }

        Logger {
            write_to_file,
            write_to_std,
            severity: severity.unwrap_or(Level::Info),
            file,
            enable_colors,
        }
    }

    /// Get color code for log level
    fn get_color(level: Level) -> &'static str {
        match level {
            Level::Error => "\x1b[31m", // Red
            Level::Warn => "\x1b[33m",  // Yellow
            Level::Info => "\x1b[36m",  // Cyan
            Level::Debug => "\x1b[35m", // Magenta
            Level::Trace => "\x1b[37m", // White
        }
    }

    /// Get reset color code
    fn get_reset() -> &'static str {
        "\x1b[0m"
    }

    /// Initialize logger with environment variables
    pub fn init() -> Result<(), log::SetLoggerError> {
        let severity = std::env::var("NORNITY_LOG")
            .or_else(|_| std::env::var("RUST_LOG"))
            .unwrap_or_else(|_| "info".to_string())
            .parse::<Level>()
            .unwrap_or(Level::Info);

        let write_to_std = Some(LogOutput::Stderr);
        let write_to_file = std::env::var("NORNITY_LOG_FILE").is_ok();
        let enable_colors = std::env::var("NO_COLOR").is_err();

        let logger = Logger::new(
            None,
            Some(severity),
            write_to_std,
            write_to_file,
            enable_colors,
        );
        log::set_max_level(LevelFilter::Trace);
        log::set_boxed_logger(Box::new(logger))?;
        Ok(())
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.severity
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let level_str = record.level().as_str();
        let _target = record.target();
        let args = record.args();

        let formatted_message = if self.enable_colors {
            let color = Self::get_color(record.level());
            let reset = Self::get_reset();
            format!("{color}[{timestamp}] {level_str}{reset} {args}\n")
        } else {
            format!("[{timestamp}] {level_str} {args}\n") // for Later debugging use {target} too 
        };

        // Write to stdout/stderr
        if let Some(write_to_std) = &self.write_to_std {
            match write_to_std {
                LogOutput::Stdout => {
                    let _ = std::io::stdout().write_all(formatted_message.as_bytes());
                }
                LogOutput::Stderr => {
                    let _ = std::io::stderr().write_all(formatted_message.as_bytes());
                }
            }
        }

        // Write to file (without colors)
        if self.write_to_file {
            if let Some(file) = &self.file {
                if let Ok(mut file_guard) = file.lock() {
                    let file_message = format!("[{timestamp}] {level_str} {args}");
                    let _ = writeln!(file_guard, "{file_message}");
                }
            }
        }
    }

    fn flush(&self) {
        let _ = std::io::stdout().flush();
        let _ = std::io::stderr().flush();
    }
}
