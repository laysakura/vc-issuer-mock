#![doc = include_str!("../README.md")]

pub(crate) mod settings;

use axum::{routing::post, Router};
use std::env;
use vc_issuer_mock_core::{vc_api_router, IssuerKeys};

use crate::settings::Settings;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = Settings::new_from_env();
    let issuer_keys = IssuerKeys::default();

    let app = Router::new()
        .nest("/vc-api", vc_api_router(issuer_keys.clone()))
        .nest("/oid4vci", vc_api_router(issuer_keys));

    let addr = ([0, 0, 0, 0], settings.port).into();
    info!("[vc-issuer-mock-http] Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
