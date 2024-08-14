//! Implements [Verifiable Credentials Data Model v2.0](https://www.w3.org/TR/vc-data-model-2.0).
//!
//! vc-issuer-mock family doesn't intend to implement the VCDM (either v1 or v2), instead, it wants to
//! rely on the ssi crate for the VCDM implementation.
//!
//! So the implementations in this module are potentially pull-requested to the ssi crate.

use ssi::{
    claims::{data_integrity, vc::v2},
    prelude::DataIntegrity,
};

/// VCDM v2 without proof.
pub type VerifiableCredentialV2 =
    v2::syntax::SpecializedJsonCredential<json_syntax::Object, (), ()>;

/// VCDM v2 with data integrity proof.
pub type VerifiableCredentialV2DataIntegrity =
    DataIntegrity<VerifiableCredentialV2, data_integrity::AnySuite>;

pub(crate) mod default_vc_properties;
pub(crate) mod problem_details;
