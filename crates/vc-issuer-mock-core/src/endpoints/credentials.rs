//! Implements the following endpoints from [VC-API](https://w3c-ccg.github.io/vc-api/):
//!
//! - `POST /credentials/issue`

use axum::Json;

use crate::endpoints::req::IssueRequest;

/// `POST /credentials/issue``
#[axum::debug_handler]
pub(crate) async fn issue(Json(req): Json<IssueRequest>) {
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
