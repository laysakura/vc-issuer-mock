// Before running the tests, you can specify a BASE_URL, such as
// BASE_URL=http://localhost:40443/zDdfsdfs npm test
const baseUrl = process.env.BASE_URL || "http://localhost:40080";
const didkitHttpBaseUrl =
  process.env.DIDKIT_HTTP_BASE_URL || "http://localhost:3000";

module.exports = {
  settings: {},
  implementations: [
    {
      name: "VC Issuer Mock",
      implementation: "vc-issuer-mock-core (local test)",
      issuers: [
        {
          id: "did:myMethod:implementation:issuer:id",
          endpoint: `${baseUrl}/credentials/issue`,
          tags: ["vc2.0"],
        },
      ],

      // Settings from: <>https://github.com/spruceid/didkit-http/blob/a10928734de046074b3dbde05bb4c3db02ce5d10/tests/localConfig.cjs#L90-L113
      verifiers: [
        {
          id: "https://spruceid.com",
          endpoint: `${didkitHttpBaseUrl}/credentials/verify`,
          supports: {
            vc: ["1.1", "2.0"],
          },
          supportedEcdsaKeyTypes: ["P-256", "P-384"],
          tags: [
            "vc-api",
            "Ed25519Signature2020",
            "JWT",
            "ecdsa-rdfc-2019",
            "ecdsa-sd-2023",
            "eddsa-rdfc-2022",
            "bbs-2023",
            "vc2.0",
          ],
        },
      ],
      vpVerifiers: [
        {
          id: "https://spruceid.com",
          endpoint: `${didkitHttpBaseUrl}/presentations/verify`,
          supports: {
            vc: ["1.1", "2.0"],
          },
          supportedEcdsaKeyTypes: ["P-256", "P-384"],
          tags: [
            "vc-api",
            "Ed25519Signature2020",
            "JWT",
            "ecdsa-rdfc-2019",
            "ecdsa-sd-2023",
            "eddsa-rdfc-2022",
            "bbs-2023",
            "vc2.0",
          ],
        },
      ],
      didResolvers: [
        {
          id: "https://spruceid.com",
          endpoint: `${didkitHttpBaseUrl}/identifiers`,
          tags: ["did-key"],
        },
      ],
    },
  ],
};
