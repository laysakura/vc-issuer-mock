use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use serde::Serialize;
use serde_with::serde_as;

/// Successful response used in vc-issuer-mock family.
#[serde_as]
#[derive(Debug, Serialize)]
pub(crate) struct SuccessRes<T: Serialize> {
    #[serde_as(as = "serde_with::FromInto<u16>")]
    pub(crate) status: StatusCode,
    pub(crate) body: T,
}

impl<T: Serialize> IntoResponse for SuccessRes<T> {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
