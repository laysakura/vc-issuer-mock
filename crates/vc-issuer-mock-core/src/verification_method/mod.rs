//! Verification methods are ways to verify digital signatures.
//! They consist of cryptographic algorithms and keys.
//!
//! In this module, verification methods are represented as JWKs.

use std::borrow::Cow;

use ssi::{
    claims::vc::syntax::{IdOr, IdentifiedObject},
    dids::{AnyDidMethod, VerificationMethodDIDResolver},
    json_ld::iref,
    jwk::Params,
    prelude::AnySuite,
    verification_methods::{
        AnyMethod, Ed25519VerificationKey2020, MaybeJwkVerificationMethod, ReferenceOrOwned,
        ReferenceOrOwnedRef, ResolutionOptions, VerificationMethodResolutionError,
        VerificationMethodResolver,
    },
    JWK,
};

use crate::{
    endpoints::res::error_res::custom_problem_types::CustomProblemType,
    vcdm_v2::problem_details::ProblemDetails, IssuerKeys,
};

/// A verification method.
#[derive(Debug)]
pub(crate) struct VerificationMethod(AnyMethod);

impl VerificationMethod {
    pub(crate) fn try_to_jwk(&self) -> Result<JWK, ProblemDetails> {
        self.0
            .try_to_jwk()
            .map(|jwk| jwk.into_owned())
            .ok_or_else(|| {
                ProblemDetails::new(
                    CustomProblemType::VerificationMethodResolutionError,
                    "verification method resolution error".to_string(),
                    format!(
                        "The resolved verification method cannot converted into JWK: {:?}",
                        self.0
                    ),
                )
            })
    }

    pub(crate) fn try_to_suite(&self) -> Result<AnySuite, ProblemDetails> {
        let any_method = ReferenceOrOwned::Owned(self.0.clone());
        let jwk = self.try_to_jwk()?;

        AnySuite::pick(&jwk, Some(&any_method)).ok_or_else(|| {
            ProblemDetails::new(
                CustomProblemType::InvalidCryptosuiteError,
                "invalid cryptosuite error".to_string(),
                format!(
                    "The resolved verification method cannot be used to select a cryptographic suite: {:?}",
                    self.0
                ),
            )
        })
    }
}

/// Verification method resolver. Currently supports DID methods or Ed25519 key method.
pub(crate) struct CustomVerificationMethodResolver {
    did_resolver: VerificationMethodDIDResolver<AnyDidMethod, AnyMethod>,
    issuer_keys: IssuerKeys,
}

impl CustomVerificationMethodResolver {
    pub(crate) fn new(issuer_keys: IssuerKeys) -> Self {
        let did_resolver =
            VerificationMethodDIDResolver::<_, AnyMethod>::new(AnyDidMethod::default());

        Self {
            did_resolver,
            issuer_keys,
        }
    }

    pub(crate) async fn resolve(
        &self,
        issuer: &IdOr<IdentifiedObject>,
    ) -> Result<VerificationMethod, ProblemDetails> {
        let vm_method = self
            .resolve_verification_method(Some(issuer.id().as_iri()), None)
            .await?;
        Ok(VerificationMethod(vm_method.into_owned()))
    }
}

impl VerificationMethodResolver for CustomVerificationMethodResolver {
    type Method = AnyMethod;

    // Very similar codes to the one in the [`didkit-http` crate](https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/src/dids.rs#L131-L183).
    async fn resolve_verification_method_with(
        &self,
        issuer: Option<&iref::Iri>,
        method: Option<ReferenceOrOwnedRef<'_, AnyMethod>>,
        options: ResolutionOptions,
    ) -> Result<Cow<AnyMethod>, VerificationMethodResolutionError> {
        match method {
            Some(method) => {
                if method.id().scheme().as_str() == "did" {
                    self.did_resolver
                        .resolve_verification_method_with(issuer, Some(method), options)
                        .await
                } else {
                    // Not a DID scheme.
                    // Some VCDM v2 tests use a non-DID issuer URI
                    let ed25519_key = self
                        .issuer_keys
                        .public_keys()
                        .iter()
                        .find_map(|j| match &j.params {
                            Params::OKP(p) => {
                                if p.curve == "Ed25519" {
                                    Some(p.try_into().unwrap())
                                } else {
                                    None
                                }
                            }
                            _ => None,
                        })
                        .ok_or_else(|| {
                            VerificationMethodResolutionError::InvalidVerificationMethod(ssi::verification_methods::InvalidVerificationMethod::UnsupportedMethodType(format!(r#"Only JWK with {{"kty":"OKP","crv":"Ed25519"}} is currently supported. Your issuer keys: {:?}"#, self.issuer_keys)))
                        })?;
                    let key = AnyMethod::Ed25519VerificationKey2020(
                        Ed25519VerificationKey2020::from_public_key(
                            method.id().to_owned(),
                            method.id().as_uri().unwrap().to_owned(),
                            ed25519_key,
                        ),
                    );
                    Ok(Cow::Owned(key))
                }
            }
            None => Err(VerificationMethodResolutionError::MissingVerificationMethod),
        }
    }
}
