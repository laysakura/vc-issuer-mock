//! Responses of VC-API endpoints.

use serde::Serialize;

use crate::vcdm_v2::VerifiableCredentialV2DataIntegrity;

/// Response body of `POST /credentials/issue`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueResponse {
    /// A JSON-LD Verifiable Credential with a proof.
    #[serde(flatten)]
    pub verifiable_credential: VerifiableCredentialV2DataIntegrity,
    // TODO EnvelopedVerifiableCredential
}

impl IssueResponse {
    pub(crate) fn new(verifiable_credential: VerifiableCredentialV2DataIntegrity) -> Self {
        Self {
            verifiable_credential,
        }
    }
}
