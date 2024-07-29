use axum::{routing::post, Router};
use std::env;
use vc_issuer_mock_core::{vc_api_router, IssuerKeys};

#[tokio::main]
async fn main() {
    let issuer_keys = IssuerKeys::default();
    let app = Router::new()
        .nest("/vc-api", vc_api_router(issuer_keys.clone()))
        .nest("/oid4vci", vc_api_router(issuer_keys));

    let port = env::var("ISSMOCK_PORT")
        .unwrap_or_else(|_| "4000".to_string())
        .parse::<u16>()
        .expect("Invalid port number");

    let addr = ([127, 0, 0, 1], port).into();
    println!("Listening on http://{}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
