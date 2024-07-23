//! Responses of VC-API endpoints.

pub(crate) mod error_res;
pub(crate) mod success_res;

use serde::Serialize;
use ssi::{
    claims::{data_integrity, vc::v2},
    prelude::DataIntegrity,
};

type VerifiableCredentialV2 = v2::syntax::SpecializedJsonCredential<json_syntax::Object, (), ()>;

/// Response body of `POST /credentials/issue`.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IssueResponse {
    /// A JSON-LD Verifiable Credential with a proof.
    pub verifiable_credential: DataIntegrity<VerifiableCredentialV2, data_integrity::AnySuite>,
    // TODO EnvelopedVerifiableCredential
}
