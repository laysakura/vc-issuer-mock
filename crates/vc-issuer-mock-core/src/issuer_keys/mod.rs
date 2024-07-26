use std::str::FromStr;

use anyhow::bail;
use derive_more::Display;
use josekit::jwk::{
    alg::{ec::EcCurve, ed::EdCurve},
    Jwk,
};
use ssi::{
    claims::SignatureError,
    verification_methods::{LocalSigner, MaybeJwkVerificationMethod, Signer},
    JWK,
};
use tracing::warn;

/// A Container of pairs of signing (private) keys and verification (public) keys held by the issuer.
///
/// Keys are represented in [JWK Set Format](https://datatracker.ietf.org/doc/html/rfc7517#section-5).
///
/// It, by default, generates sets of random JWKs with various key types. You can also add any valid JWKs.
///
/// # Example
///
/// ```
/// use vc_issuer_mock_core::IssuerKeys;
///
/// let issuer_keys = IssuerKeys::default();
/// ```
///
/// # Supported key types (`kty`)
///
/// - RSA
/// - EC
/// - OKP ([RFC 8037](https://datatracker.ietf.org/doc/html/rfc8037))
///
/// # Generated keys by default
///
/// - RSA (2048 bits)
/// - EC (P-384)
/// - OKP (Ed25519)
#[derive(Clone, Debug)]
pub struct IssuerKeys(Vec<SigningKey>);

/// A signing (private) key. It is represented as a [JWK](https://datatracker.ietf.org/doc/html/rfc7517).
#[derive(Clone, Eq, PartialEq, Debug, Display)]
pub struct SigningKey(Jwk);

/// A verification (public) key. It is represented as a [JWK](https://datatracker.ietf.org/doc/html/rfc7517).
#[derive(Clone, Eq, PartialEq, Debug, Display)]
pub struct VerificationKey(Jwk);

impl IssuerKeys {
    /// Create a new `IssuerKeys` instance from given signing keys.
    ///
    /// Each signing key must be formatted as a [JWK (RFC 7517)](https://datatracker.ietf.org/doc/html/rfc7517).
    ///
    /// # Panics
    ///
    /// This method panics if:
    ///
    /// - `signing_key_jwks` is empty.
    /// - any signing key is:
    ///   - not a valid JWK.
    ///   - not a verification (private) key.
    ///   - unsupported key type.
    ///
    /// # Example
    ///
    /// ```
    /// use vc_issuer_mock_core::IssuerKeys;
    ///
    /// let signing_key_jwks = vec![
    ///   r#"{"kty":"OKP","crv":"Ed25519","d":"nWGxne_9WmC6hEr0kuwsxERJxWl7MmkZcDusAxyuf2A","x":"11qYAYKxCrfVS_7TyWQHOg7hcvPapiMlrwIaaPcHURo"}"#,
    ///   r#"{"kty": "EC","crv": "P-384","d": "wouCtU7Nw4E8_7n5C1-xBjB4xqSb_liZhYMsy8MGgxUny6Q8NCoH9xSiviwLFfK_","ext": true,"key_ops": ["sign"],"x": "SzrRXmyI8VWFJg1dPUNbFcc9jZvjZEfH7ulKI1UkXAltd7RGWrcfFxqyGPcwu6AQ","y": "hHUag3OvDzEr0uUQND4PXHQTXP5IDGdYhJhL-WLKjnGjQAw0rNGy5V29-aV-yseW"}"#,
    /// ];
    /// let issuer_keys = IssuerKeys::new(signing_key_jwks);
    /// ```
    pub fn new<I>(signing_key_jwks: I) -> Self
    where
        I: IntoIterator,
        I::Item: AsRef<str>,
    {
        let signing_key_jwks = signing_key_jwks.into_iter().collect::<Vec<_>>();
        assert!(!signing_key_jwks.is_empty(), "empty signing keys");

        let signing_keys = signing_key_jwks
            .iter()
            .map(|jwk| {
                SigningKey::new(jwk.as_ref())
                    .expect(format!("invalid JWK: {}", jwk.as_ref()).as_str())
            })
            .collect();

        Self(signing_keys)
    }

    /// Get signing keys specified by `kid` parameter in JWK.
    ///
    /// Since [the specification allows multiple keys with the same `kid`](https://datatracker.ietf.org/doc/html/rfc7517#section-4.5),
    /// this method returns a vector.
    pub fn get_signing_key(&self, kid: &str) -> Vec<SigningKey> {
        self.0
            .iter()
            .filter(|sk| sk.0.key_id() == Some(kid))
            .cloned()
            .collect()
    }

    /// Get verification keys specified by `kid` parameter in JWK.
    ///
    /// Since [the specification allows multiple keys with the same `kid`](https://datatracker.ietf.org/doc/html/rfc7517#section-4.5),
    /// this method returns a vector.
    pub fn get_verification_key(&self, kid: &str) -> Vec<VerificationKey> {
        self.get_signing_key(kid)
            .into_iter()
            .map(|sk| VerificationKey::from(&sk))
            .collect()
    }

    /// Get the key pairs of the issuer.
    ///
    /// # Returns
    ///
    /// A vector of tuples, each containing `(signing_key, verification_key)`.
    pub fn key_pairs(&self) -> Vec<(SigningKey, VerificationKey)> {
        self.0
            .iter()
            .map(|sk| {
                let vk = VerificationKey::from(sk);
                (sk.clone(), vk)
            })
            .collect()
    }

    /// Find the signing key corresponding to the given verification key.
    pub fn find_signing_key_from(&self, verification_key: &VerificationKey) -> Option<SigningKey> {
        self.key_pairs()
            .iter()
            .find_map(|(sk, vk)| (vk == verification_key).then(|| sk.clone()))
    }

    pub(crate) fn into_local_signer(self) -> LocalSigner<Self> {
        LocalSigner(self)
    }
}

impl Default for IssuerKeys {
    fn default() -> Self {
        let jws_rsa = Jwk::generate_rsa_key(2048).unwrap();
        let jws_ec = Jwk::generate_ec_key(EcCurve::P384).unwrap();
        let jws_okp = Jwk::generate_ed_key(EdCurve::Ed25519).unwrap();
        Self::new(vec![
            &jws_rsa.to_string(),
            &jws_ec.to_string(),
            &jws_okp.to_string(),
        ])
    }
}

impl SigningKey {
    /// All keys created from user input must pass this function.
    ///
    /// # Errors
    ///
    /// - Invalid JWK
    /// - Unsupported key type
    /// - Not a private key
    fn new(signing_key_jwk: &str) -> anyhow::Result<Self> {
        let jwk =
            Jwk::from_bytes(signing_key_jwk).expect(&format!("invalid JWK: {}", signing_key_jwk));

        // validation
        match jwk.key_type() {
            "RSA" => {
                if jwk.parameter("d").is_none() {
                    bail!("(RSA) not a private key: {}", signing_key_jwk);
                }
            }
            "EC" => {
                if jwk.parameter("d").is_none() {
                    bail!("(EC) not a private key: {}", signing_key_jwk);
                }
            }
            "OKP" => {
                if jwk.parameter("d").is_none() {
                    bail!("(OKP) not a private key: {}", signing_key_jwk);
                }
            }
            ty => bail!("unsupported key type: {}", ty),
        };

        Ok(Self(jwk))
    }
}

impl VerificationKey {
    /// All keys created from user input must pass this function.
    ///
    /// # Errors
    ///
    /// - Invalid JWK
    /// - Unsupported key type
    /// - Not a public key
    fn new(verification_key_jwk: &str) -> anyhow::Result<Self> {
        let jwk = Jwk::from_bytes(verification_key_jwk)
            .expect(&format!("invalid JWK: {}", verification_key_jwk));

        // validation
        match jwk.key_type() {
            "RSA" => {
                if jwk.parameter("d").is_some() {
                    bail!("(RSA) is a private key: {}", verification_key_jwk);
                }
            }
            "EC" => {
                if jwk.parameter("d").is_some() {
                    bail!("(EC) is a private key: {}", verification_key_jwk);
                }
            }
            "OKP" => {
                if jwk.parameter("d").is_some() {
                    bail!("(OKP) is a private key: {}", verification_key_jwk);
                }
            }
            ty => bail!("unsupported key type: {}", ty),
        };

        Ok(Self(jwk))
    }
}

impl From<&SigningKey> for VerificationKey {
    fn from(sk: &SigningKey) -> Self {
        VerificationKey(
            sk.0.to_public_key()
                .expect("already created JWK, should be converted into verification (public) key"),
        )
    }
}

// Mean to use internally.
impl From<&VerificationKey> for JWK {
    fn from(vk: &VerificationKey) -> Self {
        josekit_to_ssi(&vk.0)
    }
}

// Mean to use internally.
impl From<&SigningKey> for JWK {
    fn from(sk: &SigningKey) -> Self {
        josekit_to_ssi(&sk.0)
    }
}

// Mean to use internally.
impl TryFrom<&JWK> for VerificationKey {
    type Error = anyhow::Error;

    fn try_from(jwk: &JWK) -> Result<Self, Self::Error> {
        let vk_str = jwk.to_string();
        VerificationKey::new(&vk_str)
    }
}

// Mean to use internally.
impl TryFrom<&JWK> for SigningKey {
    type Error = anyhow::Error;

    fn try_from(jwk: &JWK) -> Result<Self, Self::Error> {
        let sk_str = jwk.to_string();
        SigningKey::new(&sk_str)
    }
}

// Mean to use internally.
// Similar codes to: <https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/src/keys.rs#L19-L33>
impl<M: MaybeJwkVerificationMethod> Signer<M> for IssuerKeys {
    type MessageSigner = JWK; // signing key (private key)

    async fn for_method(
        &self,
        method: std::borrow::Cow<'_, M>,
    ) -> Result<Option<Self::MessageSigner>, SignatureError> {
        if let Some(verification_jwk) = method.try_to_jwk() {
            let vk = VerificationKey::try_from(verification_jwk.as_ref()).map_err(|e| {
                warn!("Invalid public key: {:?}", e);
                SignatureError::InvalidPublicKey
            })?;

            let sk = self.find_signing_key_from(&vk);
            let sk_jwk = sk.map(|sk| JWK::from(&sk));
            Ok(sk_jwk)
        } else {
            Ok(None)
        }
    }
}

fn josekit_to_ssi(jwk: &Jwk) -> JWK {
    let json = jwk.to_string();
    JWK::from_str(&json).expect(format!("invalid JWK: {}", json).as_str())
}

#[cfg(test)]
mod tests {
    use tracing::debug;

    use crate::{
        test_jwks::{
            JWK_EC_P384_PRIV, JWK_EC_P384_PUB, JWK_OKP_ED25519_PRIV, JWK_OKP_ED25519_PUB,
            JWK_RSA_PRIV, JWK_RSA_PUB,
        },
        test_tracing::init_tracing,
    };

    use super::*;

    #[test]
    fn test_issuer_keys_default_success() {
        init_tracing();
        let issuer_keys = IssuerKeys::default();
        debug!("IssuerKeys::defauls(): {:#?}", issuer_keys);
        assert_eq!(issuer_keys.key_pairs().len(), 3);

        // assert random keys are generated
        let issuer_keys2 = IssuerKeys::default();
        assert_eq!(issuer_keys.key_pairs(), issuer_keys.key_pairs());
        assert_ne!(issuer_keys.key_pairs(), issuer_keys2.key_pairs());
    }

    #[test]
    fn test_issuer_keys_new_success() {
        let jwks = vec![JWK_RSA_PRIV, JWK_EC_P384_PRIV, JWK_OKP_ED25519_PRIV];
        let issuer_keys = IssuerKeys::new(jwks);
        assert_eq!(issuer_keys.key_pairs().len(), 3);
    }

    #[test]
    #[should_panic]
    fn test_issuer_keys_new_panic_empty_jwks() {
        let jwks = Vec::<&str>::new();
        let _ = IssuerKeys::new(jwks);
    }

    #[test]
    #[should_panic]
    fn test_issuer_keys_new_panic_invalid_jwks() {
        let jwks = vec![r#"{"invalid_as": "jwk"}"#];
        let _ = IssuerKeys::new(jwks);
    }

    #[test]
    #[should_panic]
    fn test_issuer_keys_new_panic_rsa_pubkey() {
        let jwks = vec![JWK_RSA_PUB];
        let _ = IssuerKeys::new(jwks);
    }

    #[test]
    #[should_panic]
    fn test_issuer_keys_new_panic_ec_pubkey() {
        let jwks = vec![JWK_EC_P384_PUB];
        let _ = IssuerKeys::new(jwks);
    }

    #[test]
    #[should_panic]
    fn test_issuer_keys_new_panic_okp_pubkey() {
        let jwks = vec![JWK_OKP_ED25519_PUB];
        let _ = IssuerKeys::new(jwks);
    }
}
