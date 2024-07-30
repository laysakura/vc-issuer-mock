#![doc = include_str!("../README.md")]

pub(crate) mod endpoints;
pub(crate) mod settings;
pub(crate) mod templates;

#[cfg(test)]
pub mod test_tracing;

use axum::{
    routing::{get, post},
    Extension, Router,
};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::net::TcpListener;
use tracing::info;
use vc_issuer_mock_core::{
    endpoints::{
        oid4vci::{self, CredentialOffer, IssuerMetadata},
        vc_api,
    },
    IssuerKeys,
};

use crate::{endpoints::custom, settings::Settings, templates::init_templates};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let settings = Settings::new_from_env();
    let issuer_keys = IssuerKeys::default();
    let credential_offer = CredentialOffer::new(&settings.issuer_id);
    let templates = init_templates();
    let metadata = IssuerMetadata::new(&settings.issuer_id, &settings.oauth2_server);

    let vc_api_router = Router::new().route("/credentials/issue", post(vc_api::credentials::issue));

    let oid4vci_router = Router::new()
        .route("/credentials", post(oid4vci::credential))
        .route("/credential-offer", post(oid4vci::credential_offer))
        .route(
            "/.well-known/openid-credential-issuer",
            get(oid4vci::metadata),
        );

    let custom_router =
        Router::new().route("/credential-offer", get(custom::credential_offer::show));

    let app = Router::new()
        .nest("/vc-api", vc_api_router)
        .nest("/oid4vci", oid4vci_router)
        .nest("/custom", custom_router)
        .layer(Extension(issuer_keys))
        .layer(Extension(credential_offer))
        .layer(Extension(templates))
        .layer(Extension(metadata));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), settings.port);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Could not bind listener");
    info!("[vc-issuer-mock-http] Listening on http://{}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to start server");
}
