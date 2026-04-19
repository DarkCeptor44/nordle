// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod api;
mod utils;

use crate::{App, server::utils::words};
use anyhow::{Context, Result, anyhow};
use log::{debug, info};
use lru::LruCache;
use parking_lot::Mutex;
use std::{
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    num::NonZeroUsize,
    sync::Arc,
};
use tokio::net::TcpListener;

pub struct Service {
    cache: Mutex<LruCache<String, (u8, String)>>,
    words: Vec<&'static str>,
}

impl Debug for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Service")
            .field("cache", &self.cache)
            .finish_non_exhaustive()
    }
}

pub async fn serve(args: App) -> Result<()> {
    let service = Service {
        cache: Mutex::new(LruCache::new(
            NonZeroUsize::new(args.cache_size)
                .ok_or(anyhow!("invalid cache size: {}", args.cache_size))?,
        )),
        words: words(),
    };
    debug!("service={service:?}");

    let app = api::routes(&service).with_state(Arc::new(service));
    let addr = SocketAddr::new(
        args.host
            .parse::<IpAddr>()
            .unwrap_or(IpAddr::V4(Ipv4Addr::UNSPECIFIED)),
        args.port,
    );
    let listener = TcpListener::bind(addr)
        .await
        .context(anyhow!("failed to bind to address: {addr}"))?;
    info!("listening on http://{}:{}/", args.host, args.port);

    axum::serve(listener, app.into_make_service())
        .await
        .context("failed to serve axum app")
}
