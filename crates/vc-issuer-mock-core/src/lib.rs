#![doc = include_str!("../README.md")]

pub mod issuer_keys;
pub use issuer_keys::IssuerKeys;

pub(crate) mod endpoints;
pub(crate) mod vcdm_v2;
pub(crate) mod verification_method;

use axum::{routing::post, Extension, Router};

/// Create a new `axum::Router` implementing the [VC-API](https://w3c-ccg.github.io/vc-api/).
pub fn vc_api_router(issuer_keys: IssuerKeys) -> Router {
    Router::new()
        .route("/credentials/issue", post(endpoints::credentials::issue))
        .layer(Extension(issuer_keys))
}

#[cfg(test)]
pub mod test_issuer_keys;
#[cfg(test)]
pub mod test_tracing;
#[cfg(test)]
pub mod test_vc_json;
