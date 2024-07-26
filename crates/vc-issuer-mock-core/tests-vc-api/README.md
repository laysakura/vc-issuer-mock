# Running vc-data-model-2.0-test-suite against vc-issuer-mock-core

This directory contains necessary files to pass the [vc-data-model-2.0-test-suite](https://github.com/w3c/vc-data-model-2.0-test-suite/).

## Test locally

Run test targets:

```console
cd ../../../
docker build  -t vc-issuer-mock-core -f docker/vc-issuer-mock-core/Dockerfile .
docker run --rm --name vc-issuer-mock-core -p 3000:3000 -p 40080:40080 vc-issuer-mock-core
```

Setup tests:

```console
./local-setup.sh
```

Run tests:

```console
./local-test.sh
```
