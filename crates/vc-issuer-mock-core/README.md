# vc-issuer-mock-core

<!-- cargo-rdme start -->

Core implementations for VC Issuer Mock. Meant to be used only as a dependency for vc-issuer-mock- family.

## Conformance

See the [top-level README](https://github.com/laysakura/vc-issuer-mock/blob/main/README.md#conformance).

## Development

### Error handling

#### In `crate::endpoints::vc_api`

Use `Result<T, ProblemDetails>` for most functions. Since `ProblemDetails` requires `anyhow::Error` as a cause, `ProblemDetail` helps to add backtraces to some errors defined in other crates.

Use `Result<T, VcApiError>` for API handler functions.

`ProblemDetails::detail` are returned to clients, so it should not include any sensitive information.

## Utility bin crates

### vc-issuer-mock-core (`crate::server`)

Used as the test target for the W3C VC API conformance tests.

```console
cargo run --bin vc-issuer-mock-core --features="server"
```

### gen-keypair (`crate::gen_keypair`)

Generates key-pairs used for:

- Private key: `IssuerKeys`
- Public key: Issuer ID (`did:key`)

```console
cargo run --bin gen-keypair --features="keypair"
```

<!-- cargo-rdme end -->

### W3C test suites

[`tests-vc-api` subdirectory](./tests-vc-api/) contains documents and scripts to locally test the conformance.

See the list of statuses to conformance tests in the [top-level README](../../README.md).
