//! Responses of VC-API endpoints.

pub(crate) mod error_res;

use axum::{http::StatusCode, Json};

/// Common error response.
#[derive(Clone, Debug)]
pub(crate) struct ErrorRes<T> {
    status: StatusCode,
    body: Json<T>,
}
