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

<!-- cargo-rdme end -->

### W3C test suites

[`tests-vc-api` subdirectory](./tests-vc-api/) contains documents and scripts to locally test the conformance.

See the list of statuses to conformance tests in the [top-level README](../../README.md).
