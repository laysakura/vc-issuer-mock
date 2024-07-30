use anyhow::anyhow;
use axum::{
    async_trait,
    body::Body,
    extract::{rejection::JsonRejection, FromRequest},
};
use http::Request;

use crate::{
    endpoints::vc_api::res::error_res::VcApiErrorRes,
    vcdm_v2::problem_details::{PredefinedProblemType, ProblemDetails},
};

/// A wrapper for `axum::Json` to handle JSON parse errors.
#[derive(Clone, Debug)]
pub struct JsonReq<T>(pub(crate) T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonReq<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = VcApiErrorRes;

    async fn from_request(req: Request<Body>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(e) => {
                let problem_details = ProblemDetails::new(
                    PredefinedProblemType::ParsingError,
                    "JSON parse error".to_string(),
                    e.to_string(),
                    anyhow!("{}\nError: {:?}", e.body_text(), e),
                );
                Err(problem_details.into())
            }
        }
    }
}
