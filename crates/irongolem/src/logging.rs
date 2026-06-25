//! File-based logging for the crate.
//!
//! All diagnostic output is written to a log file rather than the console. The
//! caller controls both the verbosity (`level`) and the destination directory
//! (`directory`) through [`LogConfig`]. Emit messages with the standard [`log`]
//! macros (`log::info!`, `log::debug!`, ...) after calling [`init`].
//!
//! # Examples
//!
//! ```no_run
//! use irongolem::logging::{init, LevelFilter, LogConfig};
//!
//! let config = LogConfig {
//!     level: LevelFilter::Debug,
//!     directory: "logs".into(),
//!     ..LogConfig::default()
//! };
//! let path = init(&config).expect("logger initialised once");
//! log::info!("logging to {}", path.display());
//! ```

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub use log::LevelFilter;

/// Settings that control where log records are written and how verbose the
/// output is.
#[derive(Clone, Debug)]
pub struct LogConfig {
    /// Maximum severity that is recorded; less severe records are discarded.
    pub level: LevelFilter,
    /// Directory that holds the log file. It is created if it does not exist.
    pub directory: PathBuf,
    /// Name of the log file created inside `directory`.
    pub file_name: String,
}

impl Default for LogConfig {
    /// Returns an `Info`-level configuration that writes to `logs/irongolem.log`.
    fn default() -> Self {
        Self {
            level: LevelFilter::Info,
            directory: PathBuf::from("logs"),
            file_name: String::from("irongolem.log"),
        }
    }
}

impl LogConfig {
    /// Builds a configuration from a verbosity level and a target directory,
    /// keeping the default file name.
    pub fn new(level: LevelFilter, directory: impl Into<PathBuf>) -> Self {
        Self {
            level,
            directory: directory.into(),
            ..Self::default()
        }
    }
}

/// Error returned when the logger cannot be installed.
#[derive(Debug)]
pub enum LogError {
    /// The log directory or file could not be created or opened.
    Io(io::Error),
    /// A global logger was already installed for this process.
    AlreadyInitialized,
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::Io(err) => write!(f, "failed to set up log file: {err}"),
            LogError::AlreadyInitialized => write!(f, "a global logger is already installed"),
        }
    }
}

impl std::error::Error for LogError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            LogError::Io(err) => Some(err),
            LogError::AlreadyInitialized => None,
        }
    }
}

impl From<io::Error> for LogError {
    fn from(err: io::Error) -> Self {
        LogError::Io(err)
    }
}

/// Installs the global file logger and returns the path of the log file.
///
/// The target directory is created when missing. Records at or below
/// `config.level` are appended to the log file; nothing is written to the
/// console.
///
/// # Errors
///
/// Returns [`LogError::Io`] if the directory or file cannot be prepared, and
/// [`LogError::AlreadyInitialized`] if a global logger was already set for the
/// process.
pub fn init(config: &LogConfig) -> Result<PathBuf, LogError> {
    fs::create_dir_all(&config.directory)?;
    let path = config.directory.join(&config.file_name);
    let file = fern::log_file(&path)?;

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{timestamp} [{level}] {target}: {message}",
                timestamp = chrono::Local::now().format("%Y-%m-%dT%H:%M:%S%.3f"),
                level = record.level(),
                target = record.target(),
                message = message,
            ))
        })
        .level(config.level)
        .chain(file)
        .apply()
        .map_err(|_| LogError::AlreadyInitialized)?;

    Ok(path)
}

/// Installs the global file logger from a verbosity level and directory.
///
/// This is a convenience wrapper over [`init`] that uses the default file name.
/// See [`init`] for error semantics.
pub fn init_with(level: LevelFilter, directory: impl AsRef<Path>) -> Result<PathBuf, LogError> {
    init(&LogConfig::new(level, directory.as_ref().to_path_buf()))
}
