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
use std::process::exit;

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
        default_value_t = 100
    )]
    cache_size: usize,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let args = App::parse();
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
