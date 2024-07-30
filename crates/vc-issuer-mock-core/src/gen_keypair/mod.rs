//! Generate pairs of issuer keys.
//!
//! The output is expected to be used for:
//!
//! - Private keys (JWK)
//!   - Stored in a key manager. Deployed [servers](crate::server) use them to initialize [crate::IssuerKeys].
//! - Public keys (JWK & did:key)
//!   - `did:key`s are put into DID documents, and passed from [W3C test suites as issuer ids](https://github.com/laysakura/vc-issuer-mock/blob/main/crates/vc-issuer-mock-core/tests-vc-api/localConfig.cjs).

use vc_issuer_mock_core::IssuerKeys;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let issuer_keys = IssuerKeys::default();
    for (sk, vk) in issuer_keys.key_pairs() {
        println!("Private key (JWK): {}", sk.to_private_jwk());
        println!("Public key (JWK): {}", vk.to_public_jwk());
        println!("Public key (DID): {}", vk.to_did_key());
        println!();
    }
}
