use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{middleware, routing::post, Extension, Router};
use tokio::net::TcpListener;
use tracing::info;
use vc_issuer_mock_core::{axum_middlewares::log_req_res_body, endpoints::vc_api, IssuerKeys};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let issuer_keys = IssuerKeys::default();
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
