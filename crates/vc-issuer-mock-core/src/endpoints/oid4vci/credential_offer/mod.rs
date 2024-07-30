use axum::http::Uri;
use serde::Serialize;

/// [Credential Offer](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-credential-offer-parameters)
#[derive(Clone, Debug, Serialize)]
pub struct CredentialOffer {
    credential_issuer: String,
    credential_configuration_ids: Vec<String>,
}

impl CredentialOffer {
    /// Create a new credential offer.
    ///
    /// Currently, `credential_configuration_ids` is predefined.
    pub fn new(credential_issuer: &Uri) -> Self {
        // Hard-coded. If VC Issuer Mock provides multiple credential configurations, this should be changed.
        let credential_configuration_ids = vec!["vc-issuer-mock".to_string()];

        Self {
            credential_issuer: credential_issuer.to_string(),
            credential_configuration_ids,
        }
    }

    /// Makes a credential offer endpoint URL (`openid-credential-offer://?credential_offer=...`).
    ///
    /// See: <https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-sending-credential-offer-by>
    pub fn to_url_by_value(&self) -> String {
        let json = serde_json::to_string(self).expect("failed to serialize CredentialOffer");
        let encoded = urlencoding::encode(&json);
        format!("openid-credential-offer://?credential_offer={}", encoded)
    }
}
