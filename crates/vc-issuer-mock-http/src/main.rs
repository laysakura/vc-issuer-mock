#![doc = include_str!("../README.md")]

pub(crate) mod settings;

use axum::{
    routing::{post, Route},
    Router,
};
use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};
use tokio::net::TcpListener;
use tracing::info;
use vc_issuer_mock_core::{
    endpoints::{oid4vci, vc_api},
    IssuerKeys,
};

use crate::settings::Settings;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = Settings::new_from_env();
    let issuer_keys = IssuerKeys::default();

    let vc_api_router =
        Router::new().route("/credentials//issue", post(vc_api::credentials::issue));

    let oid4vci_router = Router::new()
        .route("/credentials", post(oid4vci::credential))
        .route("/credential-offer", post(oid4vci::credential_offer));

    let app = Router::new()
        .nest("/vc-api", vc_api_router)
        .nest("/oid4vci", oid4vci_router);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), settings.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Could not bind listener");
    info!("[vc-issuer-mock-http] Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to start server");
}
