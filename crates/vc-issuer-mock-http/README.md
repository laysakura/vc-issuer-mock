# vc-issuer-mock-http

<!-- cargo-rdme start -->

HTTP API for the VC Issuer Mock.

## Installation

### Prerequisites

You need an OAuth 2.0 Authorization Server to issue access tokens. (TODO: support fully-unauthorized mode).

[navikt/mock-oauth2-server](https://github.com/navikt/mock-oauth2-server) is a good choice for automated testing.

```console
docker run --rm -p 8080:8080 ghcr.io/navikt/mock-oauth2-server:2.1.8
```

will start a mock OAuth 2.0 Authorization Server on `http://localhost:8080`.

### Environment variables

To configure the HTTP server, set the following environment variables:

- `ISSMOCK_OAUTH2_SERVER` (**required**) (URL; string)
  - URL of an OAuth 2.0 Authorization Server. Both remote and local servers are supported.

- `ISSMOCK_PORT` (optional) (port number; integer)
  - Port number to listen on. Default is 50080.

- `ISSMOCK_ISSUER_ID` (optional) (URL; string)
  - URL of the Issuer. It maybe accessed from the mock callers (Holders or Verifiers, for example). Default: `"http://localhost:{ISSMOCK_PORT}"`.

### Run with docker

TODO

### Run with cargo

TODO use crates.io package

```console
export ISSMOCK_OAUTH2_SERVER='http://localhost:8080'
cargo run -p vc-issuer-mock-http
```

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
