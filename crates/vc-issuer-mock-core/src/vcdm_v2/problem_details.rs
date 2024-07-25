use std::fmt;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use ssi::{
    claims::data_integrity::InvalidCryptosuiteString,
    verification_methods::VerificationMethodResolutionError,
};
use thiserror::Error;

use crate::endpoints::res::error_res::custom_problem_types::CustomProblemType;

/// [Problem Details](https://www.w3.org/TR/vc-data-model-2.0/#problem-details).
#[serde_as]
#[derive(Debug, Error, Serialize)]
pub(crate) struct ProblemDetails {
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    problem_type: Box<dyn ProblemType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<i32>,

    pub(crate) title: String,
    pub(crate) detail: String,
}

impl ProblemDetails {
    pub(crate) fn new<T: ProblemType>(problem_type: T, title: String, detail: String) -> Self {
        let code = problem_type.code();
        Self {
            problem_type: Box::new(problem_type),
            code: Some(code),
            title,
            detail,
        }
    }

    /// `type` property.
    pub(crate) fn r#type(&self) -> &str {
        self.problem_type.url()
    }

    /// `code` property.
    pub(crate) fn code(&self) -> Option<i32> {
        self.code
    }
}

impl fmt::Display for ProblemDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "type={}, code={}, title={}, detail={}",
            self.problem_type,
            self.code.unwrap_or(0),
            self.title,
            self.detail
        )
    }
}

impl From<VerificationMethodResolutionError> for ProblemDetails {
    fn from(e: VerificationMethodResolutionError) -> Self {
        ProblemDetails::new(
            CustomProblemType::VerificationMethodResolutionError,
            "verification method resolution error".to_string(),
            e.to_string(),
        )
    }
}

impl From<InvalidCryptosuiteString> for ProblemDetails {
    fn from(e: InvalidCryptosuiteString) -> Self {
        ProblemDetails::new(
            CustomProblemType::InvalidCryptosuiteError,
            "invalid cryptosuite error".to_string(),
            e.to_string(),
        )
    }
}

pub(crate) trait ProblemType: fmt::Display + fmt::Debug + Send + Sync + 'static {
    fn url(&self) -> &'static str;
    fn code(&self) -> i32;
}

/// Predefined `type`s in <https://www.w3.org/TR/vc-data-model-2.0/#problem-details>.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PredefinedProblemType {
    ParsingError,
    CryptographicSecurityError,
    MalformedValueError,
    RangeError,
}

impl ProblemType for PredefinedProblemType {
    fn url(&self) -> &'static str {
        match self {
            PredefinedProblemType::ParsingError => {
                "https://www.w3.org/TR/vc-data-model#PARSING_ERROR"
            }
            PredefinedProblemType::CryptographicSecurityError => {
                "https://www.w3.org/TR/vc-data-model#CRYPTOGRAPHIC_SECURITY_ERROR"
            }
            PredefinedProblemType::MalformedValueError => {
                "https://www.w3.org/TR/vc-data-model#MALFORMED_VALUE_ERROR"
            }
            PredefinedProblemType::RangeError => "https://www.w3.org/TR/vc-data-model#RANGE_ERROR",
        }
    }

    fn code(&self) -> i32 {
        match self {
            PredefinedProblemType::ParsingError => -64,
            PredefinedProblemType::CryptographicSecurityError => -65,
            PredefinedProblemType::MalformedValueError => -66,
            PredefinedProblemType::RangeError => -67,
        }
    }
}

impl fmt::Display for PredefinedProblemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_tracing::init_tracing;

    use super::*;

    #[test]
    fn test_serialize_problem_details_parsing_error() {
        init_tracing();

        let problem = ProblemDetails::new(
            PredefinedProblemType::ParsingError,
            "Parsing Error".to_string(),
            "Failed to parse the request body.".to_string(),
        );
        let json = serde_json::to_string(&problem).expect("Failed to serialize ProblemDetails");
        assert_eq!(
            json,
            r#"{"type":"https://www.w3.org/TR/vc-data-model#PARSING_ERROR","code":-64,"title":"Parsing Error","detail":"Failed to parse the request body."}"#
        );
    }

    #[test]
    fn test_serialize_problem_details_cryptographic_security_error() {
        init_tracing();

        let problem = ProblemDetails::new(
            PredefinedProblemType::CryptographicSecurityError,
            "Cryptographic Security Error".to_string(),
            "Failed to verify the cryptographic proof.".to_string(),
        );
        let json = serde_json::to_string(&problem).expect("Failed to serialize ProblemDetails");
        assert_eq!(
            json,
            r#"{"type":"https://www.w3.org/TR/vc-data-model#CRYPTOGRAPHIC_SECURITY_ERROR","code":-65,"title":"Cryptographic Security Error","detail":"Failed to verify the cryptographic proof."}"#
        );
    }

    #[test]
    fn test_serialize_problem_details_malformed_value_error() {
        init_tracing();

        let problem = ProblemDetails::new(
            PredefinedProblemType::MalformedValueError,
            "Malformed Value Error".to_string(),
            "The request body contains a malformed value.".to_string(),
        );
        let json = serde_json::to_string(&problem).expect("Failed to serialize ProblemDetails");
        assert_eq!(
            json,
            r#"{"type":"https://www.w3.org/TR/vc-data-model#MALFORMED_VALUE_ERROR","code":-66,"title":"Malformed Value Error","detail":"The request body contains a malformed value."}"#
        );
    }

    #[test]
    fn test_serialize_problem_details_range_error() {
        init_tracing();

        let problem = ProblemDetails::new(
            PredefinedProblemType::RangeError,
            "Range Error".to_string(),
            "The request body contains a value out of range.".to_string(),
        );
        let json = serde_json::to_string(&problem).expect("Failed to serialize ProblemDetails");
        assert_eq!(
            json,
            r#"{"type":"https://www.w3.org/TR/vc-data-model#RANGE_ERROR","code":-67,"title":"Range Error","detail":"The request body contains a value out of range."}"#
        );
    }
}
