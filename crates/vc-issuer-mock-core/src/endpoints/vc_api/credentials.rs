//! Implements the following endpoints from [VC-API](https://w3c-ccg.github.io/vc-api/):
//!
//! - `POST /credentials/issue`

use anyhow::anyhow;
use axum::Extension;
use ssi::{
    claims::{
        data_integrity::{AnyInputOptions, AnySignatureOptions},
        vc::v2::Credential,
        SignatureEnvironment,
    },
    prelude::CryptographicSuite,
    verification_methods::ReferenceOrOwned,
};

use crate::{
    endpoints::vc_api::{
        req::{json_req::JsonReq, IssueRequest},
        res::{
            error_res::VcApiErrorRes, success_res::SuccessRes, IssueResponse,
            VerifiableCredentialV2DataIntegrity,
        },
    },
    vcdm_v2::problem_details::{PredefinedProblemType, ProblemDetails},
    verification_method::{CustomVerificationMethodResolver, VerificationMethod},
    IssuerKeys,
};

/// `POST /credentials/issue``
#[axum::debug_handler]
pub(crate) async fn issue(
    Extension(issuer_keys): Extension<IssuerKeys>,
    JsonReq(req): JsonReq<IssueRequest>,
) -> Result<SuccessRes<IssueResponse>, VcApiErrorRes> {
    validate_issue_request(&req)?;

    let issuer = req.credential.issuer();
    let vm_resolver = CustomVerificationMethodResolver::new(issuer_keys.clone());
    let vm = vm_resolver
        .resolve(issuer)
        .await
        .map_err(|problem_details| VcApiErrorRes {
            status: http::StatusCode::BAD_REQUEST,
            problem_details,
        })?;

    let vc = create_vc_with_data_integrity(&req, issuer_keys, &vm, &vm_resolver).await?;
    let res = IssueResponse::new(vc);
    Ok(SuccessRes {
        status: http::StatusCode::CREATED,
        body: res,
    })
}

fn validate_issue_request(req: &IssueRequest) -> Result<(), VcApiErrorRes> {
    // <https://www.w3.org/TR/vc-data-model-2.0/#credential-subject>
    // > A verifiable credential contains claims about one or more subjects.
    let sub = req.credential.credential_subjects();
    if sub.is_empty() || sub.iter().any(|s| s.is_empty()) {
        return Err(VcApiErrorRes {
            status: http::StatusCode::BAD_REQUEST,
            problem_details: ProblemDetails::new(
                PredefinedProblemType::MalformedValueError,
                "validation error (credentialSubject)".to_string(),
                "`credentialSubject` property, or any of its element, must not be empty."
                    .to_string(),
                anyhow!("`credentialSubject` property, or any of its element,  must not be empty."),
            ),
        });
    }

    Ok(())
}

async fn create_vc_with_data_integrity(
    req: &IssueRequest,
    issuer_keys: IssuerKeys,
    vm: &VerificationMethod,
    vm_resolver: &CustomVerificationMethodResolver,
) -> Result<VerifiableCredentialV2DataIntegrity, ProblemDetails> {
    let suite = vm.try_to_suite()?;

    let mut signature_options: AnySignatureOptions = Default::default();
    signature_options.mandatory_pointers =
        req.options.mandatory_pointers.clone().unwrap_or_default();

    let proof_options = AnyInputOptions {
        verification_method: Some(ReferenceOrOwned::Owned(vm.as_any_method().clone())),
        ..Default::default()
    };

    let vc = suite
        .sign_with(
            SignatureEnvironment::default(),
            req.credential.clone(),
            vm_resolver,
            issuer_keys.into_local_signer(),
            proof_options,
            signature_options,
        )
        .await?;

    Ok(vc)
}

#[cfg(test)]
mod tests {
    use ssi::{claims::vc::v2::Credential, verification_methods::ProofPurpose};

    use crate::{
        test_tracing::init_tracing,
        test_vc_json::vc_data_model_2_0_test_suite::{
            CREDENTIAL_OK, CREDENTIAL_SUBJECT_NO_CLAIMS_FAIL, README_ALUMNI,
        },
        vcdm_v2::problem_details::ProblemType as _,
    };

    use super::*;

    async fn issue_(req: IssueRequest) -> Result<SuccessRes<IssueResponse>, VcApiErrorRes> {
        init_tracing();

        let issuer_keys = Extension(IssuerKeys::default());
        let req = JsonReq(req);
        issue(issuer_keys, req.clone()).await
    }

    async fn assert_issue_with_data_integrity_proof_success(req: &str) -> anyhow::Result<()> {
        let req: IssueRequest = serde_json::from_str(req)?;

        let res = issue_(req.clone()).await?;
        assert_eq!(res.status, 201);

        let req_cred = &req.credential;
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
                &req_cred.credential_subjects(),
                &res_cred.credential_subjects(),
            );
        }

        // Assert existence and the contents of the [`proof`](https://www.w3.org/TR/vc-data-integrity/#proofs)'s
        // required properties.
        {
            let proofs = res_cred.proofs.iter().collect::<Vec<_>>();
            assert_eq!(proofs.len(), 1);
            let proof = proofs[0];

            // type
            assert_eq!(format!("{:?}", proof.type_), "JsonWebSignature2020");
            // proofPurpose
            assert!(matches!(proof.proof_purpose, ProofPurpose::Assertion));
            // proofValue
            assert!(!proof.signature.as_ref().is_empty());
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_issue_with_data_integrity_proof_success_readme_alumni() -> anyhow::Result<()> {
        assert_issue_with_data_integrity_proof_success(README_ALUMNI).await
    }

    #[tokio::test]
    async fn test_issue_with_data_integrity_proof_success_credential_ok() -> anyhow::Result<()> {
        assert_issue_with_data_integrity_proof_success(CREDENTIAL_OK).await
    }

    #[tokio::test]
    async fn test_issue_with_data_integrity_proof_error_empty_credential_subject(
    ) -> anyhow::Result<()> {
        let req: IssueRequest = serde_json::from_str(CREDENTIAL_SUBJECT_NO_CLAIMS_FAIL)?;

        let error_res = issue_(req).await.unwrap_err();
        assert_eq!(error_res.status, http::StatusCode::BAD_REQUEST);

        let problem_details = error_res.problem_details;
        assert_eq!(
            problem_details.code().unwrap(),
            PredefinedProblemType::MalformedValueError.code()
        );

        Ok(())
    }
}
