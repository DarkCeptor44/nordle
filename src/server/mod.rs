mod api;
mod utils;

use crate::server::utils::{env, init_logger, is_env, words};
use anyhow::{Context, Result, anyhow};
use log::{debug, error, info};
use lru::LruCache;
use parking_lot::Mutex;
use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroUsize,
    sync::Arc,
};
use tokio::net::TcpListener;

const ENV_CACHE_SIZE: &str = "CACHE_SIZE";
const ENV_DEBUG: &str = "DEBUG";
const ENV_HOST: &str = "HOST";
const ENV_PORT: &str = "PORT";

pub struct Service {
    debug: bool,
    cache: Mutex<LruCache<String, String>>,
    words: Vec<&'static str>,
}

impl Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service")
            .field("debug", &self.debug)
            .field("cache", &self.cache)
            .finish_non_exhaustive()
    }
}

pub async fn serve(host: &str, port: u16, cache_size: u64, debug: bool) -> Result<()> {
    let debug = is_env(ENV_DEBUG) || debug;
    let host = env(ENV_HOST, host.to_string());
    let port = env(ENV_PORT, port);
    let cache_size = env(ENV_CACHE_SIZE, cache_size);

    init_logger(debug).context("failed to initialize logger")?;

    let service = Service {
        debug,
        cache: Mutex::new(LruCache::new(
            NonZeroUsize::new(10).ok_or(anyhow!("invalid cache size: {cache_size}"))?,
        )),
        words: words(),
    };
    if let Err(e) = start(Arc::new(service), &host, port).await {
        error!("Error: {e:?}");
    }

    Ok(())
}

async fn start(service: Arc<Service>, host: &str, port: u16) -> Result<()> {
    debug!("service={service:?}");

    let app = api::routes(&service).with_state(service);
    let addr = SocketAddr::new(
        host.parse::<IpAddr>()
            .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        port,
    );
    let listener = TcpListener::bind(addr)
        .await
        .context(anyhow!("failed to bind to address: {addr}"))?;
    info!("Listening on http://{host}:{port}/");

    axum::serve(listener, app.into_make_service())
        .await
        .context("failed to serve axum app")
}
