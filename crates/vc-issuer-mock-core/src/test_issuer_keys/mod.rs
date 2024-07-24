//! Example Issuer keys (for testing).

use crate::IssuerKeys;

/// <https://datatracker.ietf.org/doc/html/rfc8037#appendix-A.1>
pub const JWK_ED25519: &str = r#"{"kty":"OKP","crv":"Ed25519","d":"nWGxne_9WmC6hEr0kuwsxERJxWl7MmkZcDusAxyuf2A","x":"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"}"#;

/// <https://datatracker.ietf.org/doc/html/rfc8037#appendix-A.1>
pub fn jwk_ed25519() -> IssuerKeys {
    IssuerKeys::new(vec![JWK_ED25519])
}
