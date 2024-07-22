//! Request parameters of VC-API endpoints.

use serde::Deserialize;
use serde_with::serde_as;
use ssi::claims::{data_integrity::JsonPointerBuf, vc::v2};

/// Request body for the [`POST /credentials/issue` endpoint](https://w3c-ccg.github.io/vc-api/#issue-credential).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct IssueRequest {
    /// Currently, we do not support VCDM v1.
    pub(crate) credential: v2::SpecializedJsonCredential,
    pub(crate) options: IssueRequestOptions,
}

/// `options` field in [`self::IssueRequest``].
#[derive(Debug, Deserialize)]
#[serde_as]
#[serde(rename_all = "camelCase")]
pub(crate) struct IssueRequestOptions {
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    pub(crate) mandatory_pointers: Option<Vec<JsonPointerBuf>>,
    // TODO add `credentialId` property
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_deserialize_issue_request() {
        let json = r#"{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://university.example/credentials/1872",
    "type": ["VerifiableCredential", "ExampleAlumniCredential"],
    "issuer": "https://university.example/issuers/565049",
    "validFrom": "2023-07-01T19:23:24Z",
    "credentialSubject": {
      "id": "did:example:ebfeb1f712ebc6f1c276e12ec21",
      "alumniOf": {
        "id": "did:example:c276e12ec21ebfeb1f712ebc6f1",
        "name": "Example University"
      }
    }
  },
  "options": {}
}"#;
        let issue_request: IssueRequest = serde_json::from_str(json).unwrap();
        assert_eq!(issue_request.credential.issuer.id(), "https://university.example/issuers/565049");
        assert!(issue_request.options.mandatory_pointers.is_none());
    }
}
