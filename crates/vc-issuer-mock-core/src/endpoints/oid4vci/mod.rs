//! OID4VCI endpoints.

pub(crate) mod credential_offer;
pub use credential_offer::CredentialOffer;
use oid4vci::credential_issuer::credential_issuer_metadata::CredentialIssuerMetadata;

use crate::{endpoints::vc_api::vc_api_error::VcApiError, IssuerKeys};
use axum::{http::Uri, Extension, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct CredentialRequest {
    credential_type: String,
    format: String,
    proof: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialResponse {
    credential: String,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialOfferRequest {
    credential_issuer: String,
    credential_configuration_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CredentialOfferResponse {
    credential_offer: String,
}

/// [Credential Issuer Metadata](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-credential-issuer-metadata).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IssuerMetadata {
    credential_issuer: String,
    authorization_servers: Vec<String>,
    credential_endpoint: String,
    /// Credential Configurations. Currently only `vc-issuer-mock` is supported.
    credential_configurations_supported: HashMap<String, CredentialConfiguration>,
}

impl IssuerMetadata {
    /// Create a new `IssuerMetadata`.
    pub fn new(credential_issuer: &Uri, authorization_server: &Uri) -> Self {
        Self {
            credential_issuer: credential_issuer.to_string(),
            authorization_servers: vec![authorization_server.to_string()],
            credential_endpoint: format!("{credential_issuer}/credentials"),
            credential_configurations_supported: vec![(
                "vc-issuer-mock".to_string(),
                CredentialConfiguration {
                    format: "ldp_vc".to_string(),
                },
            )]
            .into_iter()
            .collect(),
        }
    }
}

/// Credential Configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct CredentialConfiguration {
    format: String,
}

// TODO use another error type
#[axum::debug_handler]
pub async fn credential(
    Extension(issuer_keys): Extension<IssuerKeys>,
    Json(req): Json<CredentialRequest>,
) -> Result<Json<CredentialResponse>, VcApiError> {
    // Implement the logic for the credential endpoint
    Ok(Json(CredentialResponse {
        credential: "dummy_credential".to_string(),
    }))
}

#[axum::debug_handler]
pub async fn credential_offer(
    Extension(issuer_keys): Extension<IssuerKeys>,
    Json(req): Json<CredentialOfferRequest>,
) -> Result<Json<CredentialOfferResponse>, VcApiError> {
    // Implement the logic for the credential offer endpoint
    Ok(Json(CredentialOfferResponse {
        credential_offer: "dummy_credential_offer".to_string(),
    }))
}

/// Endpoint for Credential Issuer Metadata.
///
/// [`GET /.well-known/openid-credential-issuer`](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-credential-issuer-metadata-)
#[axum::debug_handler]
pub async fn metadata(
    Extension(metadata): Extension<CredentialIssuerMetadata>,
) -> Json<CredentialIssuerMetadata> {
    Json(metadata)
}
