#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

mod server;

use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use dotenvy::dotenv;
use std::process::exit;

use crate::server::serve;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct App {
    #[arg(
        short = 'H',
        long,
        help = "Host to listen on (env: HOST)",
        default_value = "0.0.0.0"
    )]
    host: String,

    #[arg(
        short,
        long,
        help = "Port to listen on (env: PORT)",
        default_value_t = 8080
    )]
    port: u16,

    #[arg(long, help = "Enable debug logging (env: DEBUG)", default_value_t)]
    debug: bool,

    #[arg(
        short,
        long,
        help = "Cache size (env: CACHE_SIZE)",
        default_value_t = 10
    )]
    cache_size: u64,
}

#[tokio::main]
async fn main() {
    if let Err(e) = main_impl().await {
        eprintln!("{}", format!("nordle: {e:?}").red());
        exit(1);
    }
}

async fn main_impl() -> Result<()> {
    dotenv().ok();

    let args = App::parse();

    serve(&args.host, args.port, args.cache_size, args.debug)
        .await
        .context("failed to serve backend")
}
