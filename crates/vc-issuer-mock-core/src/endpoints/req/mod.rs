//! Request parameters of VC-API endpoints.
//!
//! ["3.5 Handling Unknown Options and Data"](https://w3c-ccg.github.io/vc-api/#handling-unknown-options-and-data)
//! section says we MUST return error on unknown fields.
//! We use #[serde(deny_unknown_fields)] to enforce this.

pub(crate) mod json_req;

use serde::Deserialize;
use serde_with::serde_as;
use ssi::claims::data_integrity::JsonPointerBuf;

use crate::endpoints::res::VerifiableCredentialV2;

/// Request body for the [`POST /credentials/issue` endpoint](https://w3c-ccg.github.io/vc-api/#issue-credential).
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]

pub(crate) struct IssueRequest {
    /// Currently, we do not support VCDM v1.
    pub(crate) credential: VerifiableCredentialV2,
    pub(crate) options: IssueRequestOptions,
}

/// `options` field in [`self::IssueRequest``].
#[derive(Clone, Debug, Deserialize)]
#[serde_as]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct IssueRequestOptions {
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    pub(crate) mandatory_pointers: Option<Vec<JsonPointerBuf>>,
    pub(crate) credential_id: Option<String>,
}

#[cfg(test)]
mod tests {

    use crate::{
        test_tracing::init_tracing,
        test_vc_json::{vc_data_model_2_0_test_suite, vc_issuer_api_openapi_spec},
    };

    use super::*;

    #[test]
    fn test_deserialize_issue_request() {
        init_tracing();

        let readme_alumni: IssueRequest =
            serde_json::from_str(vc_data_model_2_0_test_suite::README_ALUMNI)
                .expect("Failed to deserialize vc_data_model_2_0_test_suite::README_ALUMNI");
        assert!(readme_alumni.options.credential_id.is_none());
        assert!(readme_alumni.options.mandatory_pointers.is_none());

        let request_sample: IssueRequest =
            serde_json::from_str(vc_issuer_api_openapi_spec::REQUEST_SAMPLE)
                .expect("Failed to deserialize vc_issuer_api_openapi_spec::REQUEST_SAMPLE");
        assert_eq!(
            request_sample.options.credential_id,
            Some("example.com/ad5d541f-db7a-4bff-97e1-d403ce403767".to_string())
        );
        let ptr = request_sample.options.mandatory_pointers.unwrap();
        assert_eq!(ptr.len(), 3);
        assert_eq!(ptr[0].as_str(), "/issuer");
        assert_eq!(ptr[1].as_str(), "/validFrom");
        assert_eq!(ptr[2].as_str(), "/validUntil");
    }
}
