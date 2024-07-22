// Before running the tests, you can specify a BASE_URL, such as
// BASE_URL=http://localhost:40443/zDdfsdfs npm test
const baseUrl = process.env.BASE_URL || "http://localhost:40080/id";

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
    },
  ],
};
