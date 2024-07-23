//! Implements [Verifiable Credentials Data Model v2.0](https://www.w3.org/TR/vc-data-model-2.0).
//!
//! vc-issuer-mock family doesn't intend to implement the VCDM (either v1 or v2), instead, it wants to
//! rely on the ssi crate for the VCDM implementation.
//!
//! So the implementations in this module are potentially pull-requested to the ssi crate.

pub(crate) mod problem_details;
