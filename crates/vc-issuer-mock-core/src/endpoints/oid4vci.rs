use axum::{Extension, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use vc_issuer_mock_core::{IssuerKeys, endpoints::res::error_res::ErrorRes};

#[derive(Serialize, Deserialize)]
struct CredentialRequest {
    credential_type: String,
    format: String,
    proof: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
struct CredentialResponse {
    credential: String,
}

#[derive(Serialize, Deserialize)]
struct CredentialOfferRequest {
    credential_issuer: String,
    credential_configuration_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct CredentialOfferResponse {
    credential_offer: String,
}

#[derive(Serialize, Deserialize)]
struct WellKnownCredentialIssuer {
    credential_issuer: String,
    authorization_servers: String,
    credential_endpoint: String,
    credential_configurations_supported: Vec<CredentialConfiguration>,
}

#[derive(Serialize, Deserialize)]
struct CredentialConfiguration {
    format: String,
}

#[axum::debug_handler]
async fn credential(
    Extension(issuer_keys): Extension<IssuerKeys>,
    Json(req): Json<CredentialRequest>,
) -> Result<Json<CredentialResponse>, ErrorRes> {
    // Implement the logic for the credential endpoint
    Ok(Json(CredentialResponse {
        credential: "dummy_credential".to_string(),
    }))
}

#[axum::debug_handler]
async fn credential_offer(
    Extension(issuer_keys): Extension<IssuerKeys>,
    Json(req): Json<CredentialOfferRequest>,
) -> Result<Json<CredentialOfferResponse>, ErrorRes> {
    // Implement the logic for the credential offer endpoint
    Ok(Json(CredentialOfferResponse {
        credential_offer: "dummy_credential_offer".to_string(),
    }))
}

#[axum::debug_handler]
async fn well_known_credential_issuer() -> Json<WellKnownCredentialIssuer> {
    // Implement the logic for the well-known credential issuer endpoint
    Json(WellKnownCredentialIssuer {
        credential_issuer: "https://github.com/laysakura/vc-issuer-mock".to_string(),
        authorization_servers: "http://localhost:???/".to_string(),
        credential_endpoint: "http://localhost:{ISSMOCK_PORT}/credential".to_string(),
        credential_configurations_supported: vec![CredentialConfiguration {
            format: "ldp_vc".to_string(),
        }],
    })
}
