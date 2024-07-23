//! Error responses of VC-API endpoints.

use std::fmt;

use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

/// [Problem Details](https://www.w3.org/TR/vc-data-model-2.0/#problem-details).
#[serde_as]
#[derive(Clone, Debug, Serialize)]
pub(crate) struct ProblemDetails {
    #[serde(rename = "type")]
    #[serde_as(as = "DisplayFromStr")]
    problem_type: ProblemType,

    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<i32>,

    pub(crate) title: String,
    pub(crate) detail: String,
}

impl ProblemDetails {
    pub(crate) fn new(problem_type: ProblemType, title: String, detail: String) -> Self {
        let code = problem_type.code();
        Self {
            problem_type,
            code: Some(code),
            title,
            detail,
        }
    }
}

#[serde_as]
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) enum ProblemType {
    ParsingError,
    CryptographicSecurityError,
    MalformedValueError,
    RangeError,
}

impl ProblemType {
    fn url(&self) -> &'static str {
        match self {
            ProblemType::ParsingError => "https://www.w3.org/TR/vc-data-model#PARSING_ERROR",
            ProblemType::CryptographicSecurityError => {
                "https://www.w3.org/TR/vc-data-model#CRYPTOGRAPHIC_SECURITY_ERROR"
            }
            ProblemType::MalformedValueError => {
                "https://www.w3.org/TR/vc-data-model#MALFORMED_VALUE_ERROR"
            }
            ProblemType::RangeError => "https://www.w3.org/TR/vc-data-model#RANGE_ERROR",
        }
    }

    fn code(&self) -> i32 {
        match self {
            ProblemType::ParsingError => -64,
            ProblemType::CryptographicSecurityError => -65,
            ProblemType::MalformedValueError => -66,
            ProblemType::RangeError => -67,
        }
    }
}

impl fmt::Display for ProblemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_problem_details_parsing_error() {
        let problem = ProblemDetails::new(
            ProblemType::ParsingError,
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
        let problem = ProblemDetails::new(
            ProblemType::CryptographicSecurityError,
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
        let problem = ProblemDetails::new(
            ProblemType::MalformedValueError,
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
        let problem = ProblemDetails::new(
            ProblemType::RangeError,
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
