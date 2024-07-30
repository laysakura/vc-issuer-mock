//! Request parameters of VC-API endpoints.
//!
//! ["3.5 Handling Unknown Options and Data"](https://w3c-ccg.github.io/vc-api/#handling-unknown-options-and-data)
//! section says we MUST return error on unknown fields.
//! We use #[serde(deny_unknown_fields)] to enforce this.

pub(crate) mod json_req;

use serde::{Deserialize, Deserializer};
use serde_json::Value;
use serde_with::{serde_as, DeserializeAs};
use ssi::claims::data_integrity::JsonPointerBuf;

use crate::{
    endpoints::vc_api::res::VerifiableCredentialV2, vcdm_v2::default_vc_properties::VC_DEFAULT_ISSUER,
};

/// Request body for the [`POST /credentials/issue` endpoint](https://w3c-ccg.github.io/vc-api/#issue-credential).
#[serde_as]
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct IssueRequest {
    /// Currently, we do not support VCDM v1.
    ///
    /// A request parameter not always contains all necessary properties as VCDM v2.
    /// For example, "issuer" property is mandatory [in VCDM v2](https://www.w3.org/TR/vc-data-model-2.0/#issuer),
    /// but not in the [VC-API's request parameter](https://w3c-ccg.github.io/vc-api/#issue-credential).
    ///
    /// [`self::VerifiableCredentialV2WithDefault`] is a wrapper struct to provide default values for missing properties.
    #[serde_as(as = "VerifiableCredentialV2WithDefault")]
    pub(crate) credential: VerifiableCredentialV2,
    #[serde(default)]
    pub(crate) options: IssueRequestOptions,
}

/// `options` field in [`self::IssueRequest``].
#[derive(Clone, Debug, Default, Deserialize)]
#[serde_as]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct IssueRequestOptions {
    #[serde_as(as = "Option<Vec<DisplayFromStr>>")]
    pub(crate) mandatory_pointers: Option<Vec<JsonPointerBuf>>,
    #[allow(dead_code)]
    pub(crate) credential_id: Option<String>,
}

struct VerifiableCredentialV2WithDefault;

impl<'de> DeserializeAs<'de, VerifiableCredentialV2> for VerifiableCredentialV2WithDefault {
    fn deserialize_as<D>(deserializer: D) -> Result<VerifiableCredentialV2, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut value: Value = Value::deserialize(deserializer)?;

        if let Value::Object(ref mut map) = value {
            if !map.contains_key("issuer") {
                map.insert(
                    "issuer".to_string(),
                    Value::String(VC_DEFAULT_ISSUER.to_string()),
                );
            }
        }

        VerifiableCredentialV2::deserialize(value).map_err(serde::de::Error::custom)
    }
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
