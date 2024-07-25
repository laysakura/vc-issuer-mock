//! Example Issuer keys (for testing).

use crate::IssuerKeys;

const JWK_P384: &str = r#"{
  "kty": "EC",
  "crv": "P-384",
  "d": "wouCtU7Nw4E8_7n5C1-xBjB4xqSb_liZhYMsy8MGgxUny6Q8NCoH9xSiviwLFfK_",
  "ext": true,
  "key_ops": ["sign"],
  "x": "SzrRXmyI8VWFJg1dPUNbFcc9jZvjZEfH7ulKI1UkXAltd7RGWrcfFxqyGPcwu6AQ",
  "y": "hHUag3OvDzEr0uUQND4PXHQTXP5IDGdYhJhL-WLKjnGjQAw0rNGy5V29-aV-yseW"
}
"#;

/// <https://developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/importKey#json_web_key>
pub fn jwk_p384() -> IssuerKeys {
    IssuerKeys::new(vec![JWK_P384])
}

const JWK_ED25519: &str = r#"{"kty":"OKP","crv":"Ed25519","d":"nWGxne_9WmC6hEr0kuwsxERJxWl7MmkZcDusAxyuf2A","x":"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"}"#;

/// <https://datatracker.ietf.org/doc/html/rfc8037#appendix-A.1>
///
/// NOTE: [Ed25519Signature2020 was deprecated in the VC-DATA-INTEGRITY spec](https://www.w3.org/TR/vc-data-integrity/#revision-history).
pub fn jwk_ed25519() -> IssuerKeys {
    IssuerKeys::new(vec![JWK_ED25519])
}
