// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod v1;

use crate::server::{Service, api::v1::ApiDocV1};
use axum::{Router, response::Html};
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn routes(_service: &Service) -> Router<Arc<Service>> {
    let r = Router::new().nest("/api/v1", v1::routes()).merge(
        SwaggerUi::new("/api/v1/docs").url("/api-docs/v1/openapi.json", ApiDocV1::openapi()),
    );

    if cfg!(debug_assertions) {
        use std::path::PathBuf;
        use tower_http::services::ServeFile;
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("assets/index.html");

        r.fallback_service(ServeFile::new(path))
    } else {
        r.fallback(|| async { Html(include_str!("../../../assets/index.html")) })
    }
}
