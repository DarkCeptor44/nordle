// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod server;

use crate::server::serve;
use anyhow::{Context, Result, anyhow};
use clap::Parser;
use dotenvy::dotenv;
use log::{LevelFilter, error, info};
use simplelog::{ColorChoice, CombinedLogger, ConfigBuilder, TermLogger, TerminalMode};
use std::{env::var, process::exit, str::FromStr};

const ENV_CACHE_SIZE: &str = "NORDLE_CACHE_SIZE";
const ENV_DEBUG: &str = "NORDLE_DEBUG";
const ENV_HOST: &str = "NORDLE_HOST";
const ENV_PORT: &str = "NORDLE_PORT";

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct App {
    #[arg(
        short = 'H',
        long,
        help = "Host to listen on",
        env = "NORDLE_HOST",
        default_value = "0.0.0.0"
    )]
    host: String,

    #[arg(
        short,
        long,
        help = "Port to listen on",
        env = "NORDLE_PORT",
        default_value_t = 8080
    )]
    port: u16,

    #[arg(
        long,
        help = "Enable debug logging",
        env = "NORDLE_DEBUG",
        default_value_t
    )]
    debug: bool,

    #[arg(
        short,
        long,
        help = "Cache size",
        env = "NORDLE_CACHE_SIZE",
        default_value_t = 10
    )]
    cache_size: u64,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut args = App::parse();
    args.debug = is_env(ENV_DEBUG) || args.debug;
    args.host = env(ENV_HOST, args.host);
    args.port = env(ENV_PORT, args.port);
    args.cache_size = env(ENV_CACHE_SIZE, args.cache_size);

    if let Err(e) = init_logger(args.debug).context("failed to init logger") {
        eprintln!("CRITICAL: {e:?}");
        exit(1);
    }

    info!(
        "\n===================================================\n------------------ nordle v{} ------------------\n===================================================\n",
        env!("CARGO_PKG_VERSION")
    );
    if let Err(e) = serve(args).await {
        error!("Application logic: {e:?}");
        exit(1);
    }
}

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
/// * `debug` - Enable debug logging
///
/// ## Errors
///
/// Returns an error if the logger could not be initialized
///
/// ## Panics
///
/// Panics if the time offset could not be set to local
fn init_logger(debug: bool) -> Result<()> {
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
