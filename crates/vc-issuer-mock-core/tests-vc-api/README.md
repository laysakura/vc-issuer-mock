# Running vc-data-model-2.0-test-suite against vc-issuer-mock-core

This directory contains necessary files to pass the [vc-data-model-2.0-test-suite](https://github.com/w3c/vc-data-model-2.0-test-suite/).

## Test locally

Run test target:

```console
# First, run didkit-http at localhost:3000

# Then run vc-issuer-mock-core
cd ../
export RUST_LOG=debug
export RUST_BACKTRACE=1  # might be too noisy
cargo run --bin vc-issuer-mock-core --features="bin"
```

Setup tests:

```console
./local-setup.sh
```

Run tests:

```console
./local-test.sh
```
