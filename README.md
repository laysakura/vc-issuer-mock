# VC Issuer Mock

Status:

- MVP of core functions implemented
- MVP of interfaces under development

---

This repository provides a testable mock implementation of a Verifiable Credential Issuer. VC Holders and Verifiers can use this mock to test their implementations.

VC Issuer Mock supports the following interfaces:

- Language-specific SDKs
  - Rust ([vc-issuer-mock-rs](./crates/vc-issuer-mock-rs/))
- HTTP API ([vc-issuer-mock-http](./crates/vc-issuer-mock-http/))

## Installation

TBD

## Usage

TBD

## Conformance

All the interfaces provided by this repository depend on [vc-issuer-mock-core](./crates/vc-issuer-mock-core/). This core library implements:

- [Verifiable Credentials Data Model 2.0](https://www.w3.org/TR/vc-data-model/) at its core.
- [VC-API](https://w3c-ccg.github.io/vc-api/) as an Issuer Service.
- **TODO** [OID4VCI (OpenID for Verifiable Credential Issuance)](https://openid.github.io/OpenID4VCI/openid-4-verifiable-credential-issuance-wg-draft.html) as an Issuer Coordinator.

The vc-issue-mock-core library is tested against the following W3C's test-suites:

- Verifiable Credentials v2.0 Test Suite ([repo](https://github.com/w3c/vc-data-model-2.0-test-suite), [report](https://w3c.github.io/vc-data-model-2.0-test-suite/))

[A document under vc-issuer-mock-core](./crates/vc-issuer-mock-core/tests-vc-api/README.md) describes how the conformance test target is built and deployed.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
