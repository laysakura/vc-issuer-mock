//! Implements [`crate::vcdm_v2::problem_details::ProblemType`].

use std::fmt;

use serde::Serialize;

use crate::vcdm_v2::problem_details::ProblemType;

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(crate) struct UnknownProblemType;

impl ProblemType for UnknownProblemType {
    fn url(&self) -> &'static str {
        "https://github.com/laysakura/vc-issuer-mock#UNKNOWN_PROBLEM_TYPE"
    }

    fn code(&self) -> i32 {
        -500
    }
}

impl fmt::Display for UnknownProblemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.url())
    }
}
