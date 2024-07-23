//! Error responses of VC-API endpoints.

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;

use crate::vcdm_v2::problem_details::ProblemDetails;

/// The error response body used in vc-issuer-mock family.
#[serde_as]
#[derive(Clone, Debug, Serialize)]
pub struct ErrorRes {
    #[serde_as(as = "serde_with::FromInto<u16>")]
    status: StatusCode,
    problem_details: ProblemDetails,
}

impl IntoResponse for ErrorRes {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}
