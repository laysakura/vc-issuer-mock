# Running vc-data-model-2.0-test-suite against vc-issuer-mock-core

This directory contains necessary files to pass the [vc-data-model-2.0-test-suite](https://github.com/w3c/vc-data-model-2.0-test-suite/).

## The test target

### Architecture

```plaintext
[HTTP client]
     |
     v (exposed port from the container)
== Docker container ==============================
     |
     v (tcp/80)
  [reverse proxy] <has /health endpoint>
     |
     +---------------------------+
     | </vc-issuer-mock/>        | </didkit-http/>
     v (tcp/40080)               v (tcp/3000)
  [vc-issuer-mock-core]    [didkit-http]
   Issuer VC-API            Verifier VC-API
```

Path under `/vc-issuer-mock/` is proxied to vc-issuer-mock-core (this crate),
and `/didkit-http/` is proxied to [didkit-http](https://github.com/spruceid/didkit-http).

The docker image is meant to exposes only port 80.

### Conformance notes

Using another implementation (didkit-http in our case) is allowed in [vc-data-model-2.0-test-suite](https://github.com/w3c/vc-data-model-2.0-test-suite/).

> If your implementation is not VC-API compatible, it is possible to "wrap" the implementation in a minimal VC-API implementation.

### Build

In the repository root:

```console
docker build  -t vc-issuer-mock-core -f docker/vc-issuer-mock-core/Dockerfile .
```

### Run locally

```console
docker run --rm --name vc-issuer-mock-core --env-file docker/vc-issuer-mock-core/env -p 8000:80 vc-issuer-mock-core
```

Call the health endpoint:

```console
curl http://localhost:8000/health
```

### Deployment

Every push to the main branch in the repo is automatically built and deployed in [Render](https://render.com/).

E.g. Health check endpoint: <https://vc-issuer-mock.onrender.com/health>

Using IaC feature (`render.yaml`) is a future work.

## Test locally

### Run test targets

After running the test target locally,

```console
./local-setup.sh
./local-test.sh
```
