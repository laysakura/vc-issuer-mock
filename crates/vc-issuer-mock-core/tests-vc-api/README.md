# Running vc-api-issuer-test-suite against vc-issuer-mock-core

This directory contains necessary files to pass the [vc-api-issuer-test-suite](https://github.com/w3c-ccg/vc-api-issuer-test-suite/).

## Test locally

Setup:

```console
cd vc-api-issuer-test-suite
npm i
cp ../.localImplementationsConfig.cjs .
sed -i "s/^const tag = 'vc-api';/const tag = 'localhost';/" tests/10-issuer.js
```

Run:

```console
npm test
```
