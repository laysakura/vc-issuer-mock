//! Implements the following endpoints from [VC-API](https://w3c-ccg.github.io/vc-api/):
//!
//! - `POST /credentials/issue`

use axum::Json;

use crate::endpoints::{
    req::IssueRequest,
    res::{error_res::ErrorRes, success_res::SuccessRes, IssueResponse},
};

/// `POST /credentials/issue``
#[axum::debug_handler]
pub(crate) async fn issue(
    Json(req): Json<IssueRequest>,
) -> Result<SuccessRes<IssueResponse>, ErrorRes> {
    todo!()
}

#[cfg(test)]
mod tests {
    use ssi::{
        claims::{
            data_integrity::TypeRef,
            vc::v2::{Context, Credential},
        },
        json_ld::IriRefBuf,
        prelude::CryptographicSuite,
        verification_methods::ProofPurpose,
    };

    use crate::{
        test_vc_json::vc_data_model_2_0_test_suite::README_ALUMNI,
        vcdm_v2::problem_details::{self, PredefinedProblemType, ProblemType},
    };

    use super::*;

    #[tokio::test]
    async fn test_issue_with_data_integrity_proof_success() -> anyhow::Result<()> {
        let req: IssueRequest = serde_json::from_str(README_ALUMNI)?;
        let req = Json(req);

        let res = issue(req.clone()).await?;
        assert_eq!(res.status, 201);

        let req_cred = &req.0.credential;
        let res_cred = &res.body.verifiable_credential;

        // Other than `proof`, the response properties should be the same as the request.
        {
            assert_eq!(
                req_cred.context.iter().collect::<Vec<_>>(),
                res_cred.context.iter().collect::<Vec<_>>(),
            );
            assert_eq!(req_cred.id(), res_cred.id());
            assert_eq!(
                req_cred.types().collect::<Vec<_>>(),
                res_cred.types().collect::<Vec<_>>()
            );
            assert_eq!(req_cred.issuer(), res_cred.issuer());

            // Although the [VC-API](https://w3c-ccg.github.io/vc-api/#issue-credential) today (2024-07-24) has
            // `issuanceDate` and `expirationDate` as response properties,
            // the [VC-Data-Model-2.0](https://www.w3.org/TR/vc-data-model-2.0/#validity-period) has
            // `validFrom` and `validUntil` instead.
            assert_eq!(req_cred.valid_from(), res_cred.valid_from());
            assert_eq!(req_cred.valid_until(), res_cred.valid_until());

            assert_eq!(
                &req_cred
                    .credential_subjects()
                    .iter()
                    .map(|neo| neo.as_object())
                    .collect::<Vec<_>>(),
                &res_cred.credential_subjects().iter().collect::<Vec<_>>()
            );
        }

        // Assert existence and the contents of the [`proof`](https://www.w3.org/TR/vc-data-integrity/#proofs)'s
        // required properties.
        {
            let proofs = res_cred.proofs.iter().collect::<Vec<_>>();
            assert_eq!(proofs.len(), 1);
            let proof = proofs[0];

            // type
            assert!(matches!(
                proof.type_.type_(),
                TypeRef::DataIntegrityProof(_)
            ));
            // proofPurpose
            assert!(matches!(proof.proof_purpose, ProofPurpose::Assertion));
            // cryptosuite
            assert!(matches!(
                proof.suite().type_(),
                TypeRef::DataIntegrityProof(_) // we do not assert the value of the cryptosuite itself
            ));
            // proofValue
            assert!(proof.signature.as_ref().len() > 0);
        }

        Ok(())
    }

    async fn assert_issue_parsing_error(req_json: &str, code: i32) -> anyhow::Result<()> {
        let req: IssueRequest = serde_json::from_str(req_json)?;
        let req = Json(req);

        let res = issue(req.clone()).await.unwrap_err();
        assert_eq!(res.status, 400);

        let problem_details = &res.problem_details;
        assert_eq!(problem_details.code().unwrap(), code);

        Ok(())
    }

    #[tokio::test]
    async fn test_issue_parsing_error() -> anyhow::Result<()> {
        assert_issue_parsing_error("{INVALID JSON}", PredefinedProblemType::ParsingError.code())
            .await
    }

    #[tokio::test]
    async fn test_issue_malformed_value_error_context_unexpected_url() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }

    #[tokio::test]
    async fn test_issue_malformed_value_error_context_not_url() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }

    /// <https://www.w3.org/TR/vc-data-model-2.0/#identifiers>
    ///
    /// > If present, the value of the id property MUST be a single URL, which MAY be dereferenceable.
    #[tokio::test]
    async fn test_issue_malformed_value_error_id_not_url() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }

    /// <https://www.w3.org/TR/vc-data-model-2.0/#types>
    ///
    /// > The value of the type property MUST be one or more terms and/or...
    #[tokio::test]
    async fn test_issue_malformed_value_error_empty_type() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }

    /// <https://www.w3.org/TR/vc-data-model-2.0/#issuer>
    ///
    /// > The value of the issuer property MUST be either a URL, or an object containing an id property whose value is a URL;
    #[tokio::test]
    async fn test_issue_malformed_value_error_issuer_not_url() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }

    /// Date and time should be separated by `T` instead of a space.
    #[tokio::test]
    async fn test_issue_malformed_value_error_valid_from_invalid() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }

    /// Date and time should be separated by `T` instead of a space.
    #[tokio::test]
    async fn test_issue_malformed_value_error_valid_until_invalid() -> anyhow::Result<()> {
        assert_issue_parsing_error(
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
            PredefinedProblemType::MalformedValueError.code(),
        )
        .await
    }
}
