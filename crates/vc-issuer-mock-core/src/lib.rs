#![doc = include_str!("../README.md")]

pub(crate) mod endpoints;

use axum::{routing::post, Router};

/// Create a new `axum::Router` implementing the [VC-API](https://w3c-ccg.github.io/vc-api/).
///
pub fn vc_api_router() -> Router {
    Router::new().route("/credentials/issue", post(endpoints::credentials::issue))
}
