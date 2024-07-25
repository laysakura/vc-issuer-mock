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
        test_issuer_keys::jwk_p384,
        test_tracing::init_tracing,
        vcdm_v2::problem_details::{PredefinedProblemType, ProblemType},
    };

    use super::*;

    use axum::{
        body::{to_bytes, Body},
        extract::Request,
        response::IntoResponse,
    };
    use serde_json::Value;
    use tower::ServiceExt;

    /// Although most of the tests to API endpoints should be done in `crate::endpoints`,
    /// tests in `crate::endpoints` require JSON serialization before calling endpoints.
    /// Thus, to test the responses of serialization-related errors, we need to test here.
    async fn test_issue_req_serialize_error<T: ProblemType>(
        req_body: &'static str,
        expected_problem_type: T,
    ) {
        init_tracing();

        let issuer_keys = jwk_p384();
        let app = vc_api_router(issuer_keys);

        let req = Request::builder()
            .method("POST")
            .uri("/credentials/issue")
            .header("content-type", "application/json")
            .body(Body::from(req_body))
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
        assert_eq!(r#type, expected_problem_type.to_string());
    }

    #[tokio::test]
    async fn test_issue_non_json_req_error_res() {
        test_issue_req_serialize_error("INVALID-AS-JSON", PredefinedProblemType::ParsingError)
            .await;
    }

    #[tokio::test]
    async fn test_issue_parsing_error_context_unexpected_url() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://example.com/INVALID_CONTEXT"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validFrom": "2023-07-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await;
    }

    #[tokio::test]
    async fn test_issue_parsing_error_context_not_url() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validFrom": "2023-07-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await;
    }

    /// <https://www.w3.org/TR/vc-data-model-2.0/#identifiers>
    ///
    /// > If present, the value of the id property MUST be a single URL, which MAY be dereferenceable.
    #[tokio::test]
    async fn test_issue_malformed_value_error_id_not_url() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "INVALID_ID",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validFrom": "2023-07-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await
    }

    /// <https://www.w3.org/TR/vc-data-model-2.0/#types>
    ///
    /// > The value of the type property MUST be one or more terms and/or...
    #[tokio::test]
    async fn test_issue_malformed_value_error_empty_type() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": [],
    "issuer": "https://university.example/issuers/565049",
    "validFrom": "2023-07-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await
    }

    /// <https://www.w3.org/TR/vc-data-model-2.0/#issuer>
    ///
    /// > The value of the issuer property MUST be either a URL, or an object containing an id property whose value is a URL;
    #[tokio::test]
    async fn test_issue_malformed_value_error_issuer_not_url() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "INVALID_ISSUER",
    "validFrom": "2023-07-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await
    }

    /// Date and time should be separated by `T` instead of a space.
    #[tokio::test]
    async fn test_issue_malformed_value_error_valid_from_invalid() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validFrom": "2023-07-01 19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await
    }

    /// Date and time should be separated by `T` instead of a space.
    #[tokio::test]
    async fn test_issue_malformed_value_error_valid_until_invalid() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validUntil": "2023-07-01 19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await
    }

    #[tokio::test]
    async fn test_issue_malformed_value_error_credential_subject_empty() {
        test_issue_req_serialize_error(
            r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validUntil": "2023-07-01T19:23:24Z",
    "credentialSubject": []
  },
  "options": {}
}"#,
            PredefinedProblemType::ParsingError,
        )
        .await
    }
}
