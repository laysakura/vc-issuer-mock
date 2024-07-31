use std::collections::HashMap;

use oid4vci::{
    credential_format_profiles::{
        w3c_verifiable_credentials::{ldp_vc, CredentialSubject},
        CredentialFormats, Parameters,
    },
    credential_issuer::{
        credential_configurations_supported::CredentialConfigurationsSupportedObject,
        credential_issuer_metadata::CredentialIssuerMetadata,
    },
};
use serde_json::json;
use url::Url;

const DEFAULT_PORT: u16 = 50080;

/// Defines the settings for the VC Issuer Mock HTTP server at launch time.
///
/// It closely tied to the [Credential Issuer Metadata](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-credential-issuer-metadata).
/// See: [self::Settings::to_credential_issuer_metadata].
#[derive(Eq, PartialEq, Debug)]
pub(crate) struct Settings {
    pub(crate) port: u16,
    issuer_id: Url,
    pub(crate) oauth2_server: Url,
}

impl Settings {
    /// Resolves the settings from the environment variables.
    ///
    /// See the [crate] document for the list of required environment variables.
    ///
    /// # Panics
    ///
    /// If any of the required environment variables is not set, or in invalid format
    pub(crate) fn new_from_env() -> Self {
        let port = std::env::var("ISSMOCK_PORT").unwrap_or_else(|_| DEFAULT_PORT.to_string());
        let port: u16 = port
            .parse()
            .expect(&format!("Invalid env ISSMOCK_PORT (`{port}`)"));

        let issuer_id = std::env::var("ISSMOCK_ISSUER_ID")
            .unwrap_or_else(|_| Self::issuer_id(port).to_string());

        let oauth2_server =
            std::env::var("ISSMOCK_OAUTH2_SERVER").expect("`ISSMOCK_OAUTH2_SERVER` env is not set");

        Self {
            port,
            issuer_id: issuer_id
                .parse()
                .expect(&format!("Invalid env ISSMOCK_ISSUER_ID (`{issuer_id}`)")),
            oauth2_server: oauth2_server.parse().expect(&format!(
                "Invalid env ISSMOCK_OAUTH2_SERVER (`{oauth2_server}`)"
            )),
        }
    }

    pub(crate) fn to_issuer_root_url(&self) -> Url {
        Self::issuer_id(self.port)
    }

    pub(crate) fn to_issuer_oid4vci_base_url(&self) -> Url {
        format!("{}oid4vci", self.to_issuer_root_url())
            .parse()
            .expect("Failed to parse the OID4VCI base URL")
    }

    fn issuer_id(port: u16) -> Url {
        format!("http://localhost:{}/", port)
            .parse()
            .expect("Failed to parse the issuer_id")
    }

    pub(crate) fn to_credential_issuer_metadata(&self) -> CredentialIssuerMetadata {
        let credential_issuer = self.to_issuer_root_url();
        let credential_endpoint = format!("{credential_issuer}/credentials").parse().unwrap();
        let supported = CredentialConfigurationsSupportedObject {
            credential_format: CredentialFormats::LdpVc(Parameters {
                parameters: (
                    ldp_vc::CredentialDefinition {
                        context: vec![
                            "https://www.w3.org/ns/credentials/v2".to_string(),
                            "https://www.w3.org/ns/credentials/examples/v2".to_string(),
                        ],
                        type_: vec![
                            "VerifiableCredential".to_string(),
                            // TODO consider what kind of VC to provide
                            "UniversityDegreeCredential".to_string(),
                        ],
                        credential_subject: CredentialSubject {
                            credential_subject: Some(json!({
                                "given_name": {
                                    "display": [
                                        {
                                            "name": "Given Name",
                                            "locale": "en-US"
                                        }
                                    ]
                                },
                                "family_name": {
                                    "display": [
                                        {
                                            "name": "Surname",
                                            "locale": "en-US"
                                        }
                                    ]
                                },
                                "degree": {},
                                "gpa": {
                                    "mandatory": true,
                                    "display": [
                                        {
                                            "name": "GPA"
                                        }
                                    ]
                                }
                            })),
                        },
                    },
                    None,
                )
                    .into(),
            }),
            scope: None,
            // TODO set did:key from IssuerKeys
            cryptographic_binding_methods_supported: vec!["did:example".to_string()],
            credential_signing_alg_values_supported: vec!["Ed25519Signature2018".to_string()],
            proof_types_supported: HashMap::new(),
            display: vec![json!({
                    "name": "University Credential",
                    "locale": "en-US",
                    "logo": {
                        "url": "https://university.example.edu/public/logo.png",
                        "alt_text": "a square logo of a university"
                    },
                    "background_color": "#12107c",
                    "text_color": "#FFFFFF"
                }
            )],
        };

        CredentialIssuerMetadata {
            credential_issuer,
            authorization_servers: vec![self.oauth2_server.clone()],
            credential_endpoint,
            credential_configurations_supported: vec![(
                "UniversityDegree_LDP_VC".to_string(),
                supported,
            )]
            .into_iter()
            .collect(),
            ..Default::default()
        }
    }
}

impl From<&Settings> for CredentialIssuerMetadata {
    fn from(settings: &Settings) -> Self {
        settings.to_credential_issuer_metadata()
    }
}
