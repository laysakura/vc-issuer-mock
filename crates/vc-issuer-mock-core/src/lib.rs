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

#[cfg(test)]
mod tests {
    use crate::{
        test_issuer_keys::jwk_p384, test_tracing::init_tracing,
        vcdm_v2::problem_details::PredefinedProblemType,
    };

    use super::*;

    use axum::{
        body::{to_bytes, Body},
        extract::Request,
        response::IntoResponse,
    };
    use serde_json::Value;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_non_json_req_error_res() {
        init_tracing();

        let issuer_keys = jwk_p384();
        let app = vc_api_router(issuer_keys);

        let req = Request::builder()
            .method("POST")
            .uri("/credentials/issue")
            .header("content-type", "application/json")
            .body(Body::from("INVALID-AS-JSON"))
            .unwrap();

        let res = app.oneshot(req).await.unwrap();

        let body = to_bytes(res.into_response().into_body(), usize::MAX)
            .await
            .unwrap();

        // `ErrorRes` does not have `Deserialize` implemented, so we need to parse the JSON manually.
        let body_str = String::from_utf8(body.to_vec()).unwrap();
        let json: Value = serde_json::from_str(&body_str).unwrap();

        let status = json["status"].as_i64().unwrap();
        assert_eq!(status, 400);

        let r#type = json["problemDetails"]["type"].as_str().unwrap();
        assert_eq!(r#type, PredefinedProblemType::ParsingError.to_string());
    }
}
