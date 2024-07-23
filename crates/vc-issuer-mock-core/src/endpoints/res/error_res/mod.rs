//! Error responses of VC-API endpoints.

pub(crate) mod custom_problem_types;

use core::fmt;
use std::error::Error;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;
use ssi::dids::document::representation::Unknown;
use tracing::error;

use crate::{
    endpoints::res::error_res::custom_problem_types::UnknownProblemType,
    vcdm_v2::problem_details::ProblemDetails,
};

/// The error response body used in vc-issuer-mock family.
#[serde_as]
#[derive(Debug, Serialize)]
pub(crate) struct ErrorRes {
    #[serde_as(as = "serde_with::FromInto<u16>")]
    status: StatusCode,
    problem_details: ProblemDetails,
}

impl Error for ErrorRes {}

impl fmt::Display for ErrorRes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "status={}, problem_details={}",
            self.status, self.problem_details
        )
    }
}

impl IntoResponse for ErrorRes {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl From<anyhow::Error> for ErrorRes {
    fn from(e: anyhow::Error) -> Self {
        error!("InternalServerError: {:?}", e);

        let problem_details = ProblemDetails::new(
            UnknownProblemType,
            "Internal Server Error".to_string(),
            e.to_string(),
        );
        ErrorRes {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            problem_details,
        }
    }
}
