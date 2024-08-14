//! A VC-API Issuer Service.
//!
//! Tested from W3C test suites.
//!
//! # Environment variables
//!
//! - `ISSMOCK_PRIV_OKP_ED25519`: Static private key (JWK) for Ed25519 (OKP).
//! - `ISSMOCK_PRIV_EC_P384`: Static private key (JWK) for P-384 (EC).
//!
//! If all of the above variables are set, the service will use them to issue VCs.
//! Otherwise, it will randomly generate key-pairs at startup.

#[cfg(feature = "server")]
pub mod log_req_res_body;

use std::{
    env,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use axum::{middleware, routing::post, Extension, Router};
use tokio::net::TcpListener;
use tracing::info;
use vc_issuer_mock_core::{axum_middlewares::log_req_res_body, endpoints::vc_api, IssuerKeys};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let issuer_keys = issuer_keys();
    let app = Router::new()
        .route("/credentials/issue", post(vc_api::credentials::issue))
        .layer(Extension(issuer_keys))
        // log req/res body
        .layer(middleware::from_fn(log_req_res_body));

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 40080);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Could not bind listener");
    info!("[vc-issuer-mock-core] Listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to start server");
}

fn issuer_keys() -> IssuerKeys {
    let sk_jwks = vec![
        env::var("ISSMOCK_PRIV_OKP_ED25519"),
        env::var("ISSMOCK_PRIV_EC_P384"),
    ]
    .into_iter()
    .collect::<Result<Vec<String>, _>>();

    let issuer_keys = sk_jwks
        .map(|sk_jwks| {
            info!("Using static issuer keys from ISSMOCK_PRIV_* env:");
            IssuerKeys::new(&sk_jwks)
        })
        .unwrap_or_else(|_| {
            info!("Using random issuer keys (not all ISSMOCK_PRIV_* envs are set):");
            IssuerKeys::default()
        });

    for (_, vk) in issuer_keys.key_pairs() {
        info!("  {}", vk.to_did_key());
    }

    issuer_keys
}
