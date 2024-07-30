//! Verification methods are ways to verify digital signatures.
//! They consist of cryptographic algorithms and keys.
//!
//! In this module, verification methods are represented as JWKs.

use std::borrow::Cow;

use anyhow::anyhow;
use ssi::{
    claims::vc::syntax::{IdOr, IdentifiedObject},
    dids::{AnyDidMethod, DIDResolver, VerificationMethodDIDResolver, DID},
    json_ld::iref,
    prelude::AnySuite,
    verification_methods::{
        AnyMethod, GenericVerificationMethod, InvalidVerificationMethod, JsonWebKey2020,
        MaybeJwkVerificationMethod, ReferenceOrOwned, ReferenceOrOwnedRef, ResolutionOptions,
        VerificationMethodResolutionError, VerificationMethodResolver,
    },
    JWK,
};

use crate::{
    endpoints::vc_api::res::error_res::custom_problem_types::CustomProblemType,
    vcdm_v2::problem_details::ProblemDetails, IssuerKeys,
};

/// A verification method.
#[derive(Debug)]
pub(crate) struct VerificationMethod(AnyMethod);

impl VerificationMethod {
    pub(crate) fn as_any_method(&self) -> &AnyMethod {
        &self.0
    }

    pub(crate) fn try_to_jwk(&self) -> Result<JWK, ProblemDetails> {
        self.0
            .try_to_jwk()
            .map(|jwk| jwk.into_owned())
            .ok_or_else(|| {
                ProblemDetails::new(
                    CustomProblemType::VerificationMethodResolutionError,
                    "verification method resolution error".to_string(),
                    "resolved verification method cannot converted into JWK".to_string(),
                    anyhow!(
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
                "The resolved verification method cannot be used to select a cryptographic suite".to_string(),
                anyhow!(
                    "The resolved verification method cannot be used to select a cryptographic suite: {:?}",
                    self.0
                ),
            )
        })
    }
}

/// Verification method resolver. Currently supports DID or JWK methods.
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
            .resolve_verification_method(
                Some(issuer.id().as_iri()),
                // これNoneだとエラーだわ
                None,
            )
            .await?;
        Ok(VerificationMethod(vm_method.into_owned()))
    }

    // Similar codes to: <https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/src/dids.rs#L131-L183>.
    async fn resolve_by_method(
        &self,
        issuer: Option<&iref::Iri>,
        method: ReferenceOrOwnedRef<'_, AnyMethod>,
        options: ResolutionOptions,
    ) -> Result<Cow<AnyMethod>, VerificationMethodResolutionError> {
        if method.id().scheme().as_str() == "did" {
            if let Ok(method) = self
                .did_resolver
                .resolve_verification_method_with(issuer, Some(method), options)
                .await
            {
                return Ok(method);
            }
        }
        self.resolve_to_jwk2020(method.id())
    }

    // Similar codes to: <https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/src/credentials.rs#L91-L121>
    async fn resolve_by_issuer(
        &self,
        issuer: &iref::Iri,
    ) -> Result<Cow<AnyMethod>, VerificationMethodResolutionError> {
        if let Ok(did) = DID::new(issuer) {
            let output = self.did_resolver.resolve(did).await.map_err(|e| {
                VerificationMethodResolutionError::InternalError(format!(
                    "Could not fetch issuer DID document `{}`: {:?}",
                    did, e
                ))
            })?;

            let vm = output
                .document
                .into_document()
                .into_any_verification_method()
                .ok_or_else(|| {
                    VerificationMethodResolutionError::InternalError(
                        "Could not get any verification method for issuer DID document".to_string(),
                    )
                })?;

            let vm = AnyMethod::try_from(GenericVerificationMethod::from(vm))?;
            Ok(Cow::<AnyMethod>::Owned(vm))
        } else {
            self.resolve_to_jwk2020(issuer)
        }
    }

    fn resolve_to_jwk2020(
        &self,
        method_or_issuer_id: &iref::Iri,
    ) -> Result<Cow<AnyMethod>, VerificationMethodResolutionError> {
        let controller = method_or_issuer_id
            .as_uri()
            .ok_or_else(|| {
                VerificationMethodResolutionError::InvalidVerificationMethod(
                    InvalidVerificationMethod::InvalidIri(method_or_issuer_id.to_string()),
                )
            })?
            .to_owned();

        // Pick the first issuer key (JWK)
        let public_key = self
            .issuer_keys
            .key_pairs()
            .first()
            .map(|(_, vk)| JWK::from(vk))
            .ok_or_else(|| {
                VerificationMethodResolutionError::InvalidVerificationMethod(
                    ssi::verification_methods::InvalidVerificationMethod::UnsupportedMethodType(
                        "No issuer keys found".to_string(),
                    ),
                )
            })?;

        let vm = AnyMethod::JsonWebKey2020(JsonWebKey2020 {
            id: method_or_issuer_id.to_owned(),
            controller,
            public_key: Box::new(public_key),
        });
        Ok(Cow::Owned(vm))
    }
}

impl VerificationMethodResolver for CustomVerificationMethodResolver {
    type Method = AnyMethod;

    // Similar codes to:
    // - <https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/src/dids.rs#L131-L183>.
    // - <https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/src/credentials.rs#L91-L121>
    async fn resolve_verification_method_with(
        &self,
        issuer: Option<&iref::Iri>,
        method: Option<ReferenceOrOwnedRef<'_, AnyMethod>>,
        options: ResolutionOptions,
    ) -> Result<Cow<AnyMethod>, VerificationMethodResolutionError> {
        match (method, issuer) {
            (Some(method), _) => self.resolve_by_method(issuer, method, options).await,
            (None, Some(issuer)) => self.resolve_by_issuer(issuer).await,
            (None, None) => Err(VerificationMethodResolutionError::MissingVerificationMethod),
        }
    }
}
