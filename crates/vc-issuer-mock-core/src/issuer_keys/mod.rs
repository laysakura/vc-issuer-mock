use std::str::FromStr;

use ssi::JWK;

/// A container of issuer keys (both private and public).
#[derive(Clone, Debug)]
pub struct IssuerKeys(Vec<IssuerKey>);

/// A key-pair of issuer key.
#[derive(Clone, Debug)]
struct IssuerKey {
    private: JWK,
    public: JWK,
}

impl IssuerKeys {
    /// Create a new `IssuerKeys` instance from given private keys.
    ///
    /// Each private key must be formatted as a [JWK (RFC 7517)](https://datatracker.ietf.org/doc/html/rfc7517).
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - `private_key_jwks` is empty.
    /// - any private key is not a valid JWK.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_issuer_mock_core::IssuerKeys;
    ///
    /// let private_key_jwks = vec![
    ///   r#"{"kty":"RSA","n": "0vx7agoebGcQSuuPiLJXZptN9nndrQmbXEps2aiAFbWhM78LhWx4cbbfAAtVT86zwu1RK7aPFFxuhDR1L6tSoc_BJECPebWKRXjBZCiFV4n3oknjhMstn64tZ_2W-5JsGY4Hc5n9yBXArwl93lqt7_RN5w6Cf0h4QyQ5v-65YGjQR0_FDW2QvzqY368QQMicAtaSqzs8KJZgnYb9c7d0zgdAZHzu6qMQvRL5hajrn1n91CbOpbISD08qNLyrdkt-bFTWhAI4vMQFh6WeZu0fM4lFd2NcRwr3XPksINHaQ-G_xBniIqbw0Ls1jF44-csFCur-kEgU8awapJzKnqDKgw","e":"AQAB","alg":"RS256","kid":"2011-04-29"}"#,
    ///   r#"{"kty":"OKP","crv":"Ed25519","d":"nWGxne_9WmC6hEr0kuwsxERJxWl7MmkZcDusAxyuf2A","x":"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"}"#,
    /// ];
    /// let issuer_keys = IssuerKeys::new(private_key_jwks);
    /// ```
    pub fn new<I>(private_key_jwks: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let private_key_jwks = private_key_jwks.into_iter().collect::<Vec<_>>();
        assert!(!private_key_jwks.is_empty());

        let issuer_keys = private_key_jwks
            .into_iter()
            .map(|jwk| {
                let private =
                    JWK::from_str(jwk.as_ref()).expect(&format!("invalid JWK: {}", jwk.as_ref()));
                let public = private.to_public();
                IssuerKey { private, public }
            })
            .collect();

        Self(issuer_keys)
    }

    /// Get the public keys of the issuer.
    pub fn public_keys(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|key| key.public.to_string())
            .collect::<Vec<_>>()
    }

    /// Get the key pairs of the issuer.
    ///
    /// # Returns
    ///
    /// A vector of tuples, each containing `(private_key, public_key)`.
    pub fn key_pairs(&self) -> Vec<(String, String)> {
        self.0
            .iter()
            .map(|key| (key.private.to_string(), key.public.to_string()))
            .collect::<Vec<_>>()
    }
}
