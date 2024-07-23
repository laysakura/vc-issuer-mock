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
        claims::{data_integrity::TypeRef, vc::v2::Credential},
        prelude::CryptographicSuite,
        verification_methods::ProofPurpose,
    };

    use crate::test_vc_json::vc_data_model_2_0_test_suite::README_ALUMNI;

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
}
