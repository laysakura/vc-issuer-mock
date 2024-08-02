//! Implements [`crate::vcdm_v2::problem_details::ProblemType`].

use std::fmt;

use serde::Serialize;

use crate::vcdm_v2::problem_details::ProblemType;

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum CustomProblemType {
    InvalidCryptosuiteError,
    VerificationMethodResolutionError,
    SignatureError,
    UnknownError,
}

impl ProblemType for CustomProblemType {
    fn url(&self) -> &'static str {
        match self {
            CustomProblemType::InvalidCryptosuiteError => {
                "https://github.com/laysakura/vc-issuer-mock#INVALID_CRYPTOSUITE_ERROR"
            }
            CustomProblemType::VerificationMethodResolutionError => {
                "https://github.com/laysakura/vc-issuer-mock#VERIFICATION_METHOD_RESOLUTION_ERROR"
            }
            CustomProblemType::SignatureError => {
                "https://github.com/laysakura/vc-issuer-mock#SIGNATURE_ERROR"
            }
            CustomProblemType::UnknownError => {
                "https://github.com/laysakura/vc-issuer-mock#UNKNOWN_ERROR"
            }
        }
    }

    fn code(&self) -> i32 {
        match self {
            CustomProblemType::InvalidCryptosuiteError => -400,
            CustomProblemType::VerificationMethodResolutionError => -401,
            CustomProblemType::SignatureError => -402,
            CustomProblemType::UnknownError => -500,
        }
    }
}

impl fmt::Display for CustomProblemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}
