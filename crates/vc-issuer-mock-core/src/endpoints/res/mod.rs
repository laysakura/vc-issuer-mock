//! Responses of VC-API endpoints.

pub(crate) mod error_res;
pub(crate) mod success_res;

use serde::Serialize;
use ssi::{
    claims::{data_integrity, vc::v2},
    prelude::DataIntegrity,
};

pub(crate) type VerifiableCredentialV2 =
    v2::syntax::SpecializedJsonCredential<json_syntax::Object, (), ()>;
pub(crate) type VerifiableCredentialV2DataIntegrity =
    DataIntegrity<VerifiableCredentialV2, data_integrity::AnySuite>;

/// Response body of `POST /credentials/issue`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IssueResponse {
    /// A JSON-LD Verifiable Credential with a proof.
    pub(crate) verifiable_credential: VerifiableCredentialV2DataIntegrity,
    // TODO EnvelopedVerifiableCredential
}

impl IssueResponse {
    pub(crate) fn new(verifiable_credential: VerifiableCredentialV2DataIntegrity) -> Self {
        Self {
            verifiable_credential,
        }
    }
}
