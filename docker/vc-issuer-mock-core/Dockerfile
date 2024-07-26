# vc-issuer-mock-core only provides Issuer Service's VC-API.
# To pass test suites like vc-data-model-2.0-test-suite, we use didkit-http for other VC-APIs.
#
# This Dockerfile builds didkit-http and vc-issuer-mock-core, and runs them together.

# Layer1: Build didkit-http and vc-issuer-mock-core
FROM rust:1.80 as builder

# Clone and build didkit-http
WORKDIR /usr/src/app
RUN git clone https://github.com/spruceid/didkit-http.git && \
    cd didkit-http && \
    git checkout 95eb4fcabd8bad58cf4de636b8ba67386c092a89 && \
    cargo build

# COPY and build vc-issuer-mock-core
COPY . /usr/src/app/vc-issuer-mock
WORKDIR /usr/src/app/vc-issuer-mock
RUN cargo build -p vc-issuer-mock-core --bin vc-issuer-mock-core --features="bin"

# Layer2: Copy built binaries and run them in lightweight container
FROM debian:bookworm-slim

# libssl3 is required for didkit-http
RUN apt-get update && apt-get install -y libssl3 && rm -rf /var/lib/apt/lists/*

# Copy built binaries
COPY --from=builder /usr/src/app/didkit-http/target/debug/didkit-http /usr/local/bin/
COPY --from=builder /usr/src/app/vc-issuer-mock/target/debug/vc-issuer-mock-core /usr/local/bin/

# Copy run script and config file
COPY docker/vc-issuer-mock-core/run_servers.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/run_servers.sh
COPY docker/vc-issuer-mock-core/didkit_http.toml /app/

# Run the script
WORKDIR /app
ENV RUST_LOG=debug
CMD ["/usr/local/bin/run_servers.sh"]
