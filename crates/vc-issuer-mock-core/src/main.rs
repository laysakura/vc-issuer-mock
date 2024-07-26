use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;
use tracing::info;
use vc_issuer_mock_core::IssuerKeys;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let issuer_keys = IssuerKeys::default();
    let app = vc_issuer_mock_core::vc_api_router(issuer_keys);

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 40080);
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Could not bind listener");
    info!("listening on {}", addr);
    axum::serve(listener, app.into_make_service())
        .await
        .expect("failed to start server");
}
