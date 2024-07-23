//! Implements the following endpoints from [VC-API](https://w3c-ccg.github.io/vc-api/):
//!
//! - `POST /credentials/issue`

use axum::Json;

use crate::endpoints::{
    req::IssueRequest,
    res::{error_res::ErrorRes, success_res::SuccessRes, IssueResponse},
};

/// `POST /credentials/issue``
#[axum::debug_handler]
pub(crate) async fn issue(
    Json(req): Json<IssueRequest>,
) -> Result<SuccessRes<IssueResponse>, ErrorRes> {
    todo!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_issue() {
//         let req =
//     }
// }
