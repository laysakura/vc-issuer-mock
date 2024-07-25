//! Error responses of VC-API endpoints.

pub(crate) mod custom_problem_types;

use core::fmt;

use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;
use thiserror::Error;
use tracing::{debug, error};

use crate::{
    endpoints::res::error_res::custom_problem_types::CustomProblemType,
    vcdm_v2::problem_details::{ProblemDetails, ProblemType},
};

/// The error response body used in vc-issuer-mock family.
#[serde_as]
#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ErrorRes {
    #[serde_as(as = "serde_with::FromInto<u16>")]
    pub(crate) status: StatusCode,
    pub(crate) problem_details: ProblemDetails,
}

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

impl From<ProblemDetails> for ErrorRes {
    fn from(problem_details: ProblemDetails) -> Self {
        let code = problem_details.code().unwrap_or(0);

        let status = if code == CustomProblemType::UnknownError.code() {
            error!("InternalServerError: {:?}", problem_details);
            StatusCode::INTERNAL_SERVER_ERROR
        } else {
            debug!("BadRequest: {:?}", problem_details);
            StatusCode::BAD_REQUEST
        };

        Self {
            status,
            problem_details,
        }
    }
}

impl From<anyhow::Error> for ErrorRes {
    fn from(e: anyhow::Error) -> Self {
        error!("InternalServerError: {:?}", e);

        let problem_details = ProblemDetails::new(
            CustomProblemType::UnknownError,
            "Internal Server Error".to_string(),
            e.to_string(),
            e,
        );
        ErrorRes {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            problem_details,
        }
    }
}
