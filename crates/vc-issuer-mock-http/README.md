# vc-issuer-mock-http

<!-- cargo-rdme start -->

HTTP API for the VC Issuer Mock.

## Installation

### Prerequisites

You need an OAuth 2.0 Authorization Server to issue access tokens. (TODO: support fully-unauthorized mode).

TODO write some recommendations.

### Environment variables

To configure the HTTP server, set the following environment variables:

- `ISSMOCK_OAUTH2_SERVER` (**required**) (URL; string)
  - URL of an OAuth 2.0 Authorization Server.
   Both remote and local servers are supported.

- `ISSMOCK_PORT` (optional) (port number; integer)
  - Port number to listen on. Default is 50080.

- `ISSMOCK_ISSUER_ID` (optional) (URL; string)
  - URL of the Issuer ID. Default is `"https://github.com/laysakura/vc-issuer-mock"`.

## Endpoints

### OID4VCI (OpenID for Verifiable Credential Issuance)

- **TODO** `openid-credential-offer://?credential_offer=...` (unauthorized)
  - The [Credential Offer Endpoint](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#section-4.1.2)
  - Wallets use this endpoint to initiate the issuance process.

- **TODO** `POST /oid4vci/credential` (authorized)
  - The [Credential Endpoint](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html#name-credential-endpoint)

### VC-API (Verifiable Credentials API)

- `POST /vc-api/credentials/issue`
  - The [Issue Credential](https://w3c-ccg.github.io/vc-api/#issue-credential) endpoint.
  - Issuers use this endpoint to issue a Verifiable Credential.

### Custom endpoints

- **TODO** `GET /custom/credential-offer`
  - An HTML page to show the Credential Offer Endpoint URLs (in a QR code and a hyperlink).

<!-- cargo-rdme end -->
