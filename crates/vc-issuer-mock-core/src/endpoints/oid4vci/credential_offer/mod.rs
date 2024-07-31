use oid4vci::credential_offer::{
    CredentialOffer as oid4vci_CredentialOffer, CredentialOfferParameters,
};
use url::Url;

/// [Credential Offer](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-credential-offer-parameters)
#[derive(Clone, Debug)]
pub struct CredentialOffer {
    inner: oid4vci_CredentialOffer,
}

impl CredentialOffer {
    /// Create a new credential offer.
    ///
    /// Currently, `credential_configuration_ids` is predefined.
    pub fn new(credential_issuer: &Url) -> Self {
        // Hard-coded. If VC Issuer Mock provides multiple credential configurations, this should be changed.
        let credential_configuration_ids = vec!["UniversityDegree_LDP_VC".to_string()];

        let parameters = CredentialOfferParameters {
            credential_issuer: credential_issuer.clone(),
            credential_configuration_ids,
            grants: None,
        };

        Self {
            inner: oid4vci_CredentialOffer::CredentialOffer(Box::new(parameters)),
        }
    }

    /// Makes a credential offer endpoint URL (`openid-credential-offer://?credential_offer=...`).
    ///
    /// See: <https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-sending-credential-offer-by>
    pub fn to_url_by_value(&self) -> String {
        self.inner.to_string()
    }
}
