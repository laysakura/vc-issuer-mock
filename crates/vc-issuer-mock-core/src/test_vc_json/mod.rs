//! Example VCs in JSON format (for testing).

/// From <https://github.com/w3c/vc-data-model-2.0-test-suite/> repo.
pub mod vc_data_model_2_0_test_suite {
    /// https://github.com/w3c/vc-data-model-2.0-test-suite/blob/b2bd2dbdad5d810cb7a85c863fdff080381667db/README.md#setup
    pub const README_ALUMNI: &str = r#"
{
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

    /// https://github.com/w3c/vc-data-model-2.0-test-suite/blob/00fa18a7b676959e7aa5e4b8774a7ce049f9c0d0/tests/input/credential-ok.json
    pub const CREDENTIAL_OK: &str = r#"
{"credential": {
  "@context": [
    "https://www.w3.org/ns/credentials/v2"
  ],
  "type": [
    "VerifiableCredential"
  ],
  "credentialSubject": {
    "id": "did:example:subject"
  }
}}"#;

    /// https://github.com/w3c/vc-data-model-2.0-test-suite/blob/00fa18a7b676959e7aa5e4b8774a7ce049f9c0d0/tests/input/credential-subject-no-claims-fail.json
    pub const CREDENTIAL_SUBJECT_NO_CLAIMS_FAIL: &str = r#"
{"credential": {
  "@context": [
    "https://www.w3.org/ns/credentials/v2"
  ],
  "type": [
    "VerifiableCredential"
  ],
  "credentialSubject": {}
}}"#;
}

pub mod vc_issuer_api_openapi_spec {
    /// From <https://w3c-ccg.github.io/vc-api/issuer.html>.
    /// Modified to use v2.
    pub const REQUEST_SAMPLE: &str = r#"
{
  "credential": {
    "@context": [
      "https://www.w3.org/ns/credentials/v2",
      "https://www.w3.org/ns/credentials/examples/v2"
    ],
    "id": "http://example.gov/credentials/3732",
    "type": [
      "VerifiableCredential",
      "UniversityDegreeCredential"
    ],
    "issuer": "did:example:123",
    "issuanceDate": "2020-03-16T22:37:26.544Z",
    "credentialSubject": {
      "id": "did:example:123",
      "degree": {
        "type": "BachelorDegree",
        "name": "Bachelor of Science and Arts"
      }
    }
  },
  "options": {
    "credentialId": "example.com/ad5d541f-db7a-4bff-97e1-d403ce403767",
    "mandatoryPointers": [
      "/issuer",
      "/validFrom",
      "/validUntil"
    ]
  }
}"#;
}
