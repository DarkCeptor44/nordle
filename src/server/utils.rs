use anyhow::{Result, anyhow};
use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};
use std::{env::var, str::FromStr};

const ANSWERS: &str = include_str!("../../assets/answers.txt");
const ALLOWED: &str = include_str!("../../assets/allowed.txt");

/// Get an environment variable or return a default value
///
/// ## Arguments
///
/// * `key` - The name of the environment variable
/// * `default` - The default value to return if the environment variable is not set
///
/// ## Returns
///
/// * `T` - The type of the environment variable
#[must_use]
pub fn env<S, T>(key: S, default: T) -> T
where
    S: AsRef<str>,
    T: FromStr,
{
    let value = var(key.as_ref()).ok().and_then(|s| s.parse().ok());
    value.unwrap_or(default)
}

/// Initialize the logger
///
/// ## Arguments
///
/// * `debug` - Whether to enable debug logging
///
/// ## Errors
///
/// Returns an error if the logger could not be initialized
///
/// ## Panics
///
/// Panics if the time offset could not be set to local
pub fn init_logger(debug: bool) -> Result<()> {
    let filter = if debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    let config = ConfigBuilder::new()
        .set_time_offset_to_local()
        .expect("could not determine local time offset")
        .set_time_format_rfc3339()
        .build();

    CombinedLogger::init(vec![TermLogger::new(
        filter,
        config,
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .map_err(|e| anyhow!(e))
}

/// Check if an environment variable is set
///
/// ## Arguments
///
/// * `name` - The name of the environment variable
///
/// ## Returns
///
/// * `bool` - `true` if the environment variable is set, `false` otherwise
#[must_use]
pub fn is_env<S>(name: S) -> bool
where
    S: AsRef<str>,
{
    !var(name.as_ref()).unwrap_or_default().trim().is_empty()
}

/// Check if a word is valid
///
/// ## Arguments
///
/// * `word` - The word to check
///
/// ## Returns
///
/// * `bool` - `true` if the word is valid, `false` otherwise
#[must_use]
pub fn is_valid_word<S>(word: S) -> bool
where
    S: AsRef<str>,
{
    let w = word.as_ref().trim();
    ANSWERS.contains(w) || ALLOWED.contains(w)
}

/// Get a list of possible answers
///
/// ## Returns
///
/// * `Vec<&'static str>` - A list of possible answers
#[must_use]
pub fn words() -> Vec<&'static str> {
    ANSWERS.lines().collect()
}
