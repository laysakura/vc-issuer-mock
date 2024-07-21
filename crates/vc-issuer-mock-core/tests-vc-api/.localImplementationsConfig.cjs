module.exports = [
  {
    name: "VC Issuer Mock",
    implementation: "vc-issuer-mock-core (local test)",
    issuers: [
      {
        id: "urn:uuid:my:implementation:issuer:id",
        endpoint: "https://localhost:40443/issuers/foo/credentials/issue",
        tags: ["vc-api", "localhost"],
      },
    ],
    verifiers: [
      {
        id: "https://localhost:40443/verifiers/z19uokPn3b1Z4XDbQSHo7VhFR",
        endpoint:
          "https://localhost:40443/verifiers/z19uokPn3b1Z4XDbQSHo7VhFR/credentials/verify",
        tags: ["vc-api", "localhost"],
      },
    ],
  },
];
