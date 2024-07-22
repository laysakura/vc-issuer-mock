module.exports = [
  {
    name: "VC Issuer Mock",
    implementation: "vc-issuer-mock-core (local test)",
    issuers: [
      {
        id: "urn:uuid:my:implementation:issuer:id",
        endpoint: "http://localhost:40080/credentials/issue",
        tags: ["vc-api", "localhost"],
      },
    ],
    verifiers: [
      {
        id: "http://localhost:40080/verifiers/z19uokPn3b1Z4XDbQSHo7VhFR",
        endpoint:
          "http://localhost:40080/verifiers/z19uokPn3b1Z4XDbQSHo7VhFR/credentials/verify",
        tags: ["vc-api", "localhost"],
      },
    ],
  },
];
