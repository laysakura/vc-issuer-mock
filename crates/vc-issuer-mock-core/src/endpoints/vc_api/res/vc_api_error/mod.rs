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
    endpoints::vc_api::res::vc_api_error::custom_problem_types::CustomProblemType,
    vcdm_v2::problem_details::{ProblemDetails, ProblemType},
};

/// The error response body used in VC-API.
#[serde_as]
#[derive(Debug, Error, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VcApiError {
    #[serde_as(as = "serde_with::FromInto<u16>")]
    pub(crate) status: StatusCode,
    pub(crate) problem_details: ProblemDetails,
}

impl fmt::Display for VcApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "status={}, problem_details={}",
            self.status, self.problem_details
        )
    }
}

impl IntoResponse for VcApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl From<ProblemDetails> for VcApiError {
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

impl From<anyhow::Error> for VcApiError {
    fn from(e: anyhow::Error) -> Self {
        error!("InternalServerError: {:?}", e);

        let problem_details = ProblemDetails::new(
            CustomProblemType::UnknownError,
            "Internal Server Error".to_string(),
            e.to_string(),
            e,
        );
        VcApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            problem_details,
        }
    }
}
