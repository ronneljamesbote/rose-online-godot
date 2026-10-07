# ROSE game server: SpacetimeDB with the rose module, in one container that sets itself up.
# Build and run from the repository root:  docker compose up -d
# See server/README.md.

FROM rust:1-slim-bookworm AS module
RUN rustup target add wasm32-unknown-unknown
WORKDIR /src
COPY module module
COPY data/zone1 data/zone1
RUN cd module && cargo build --release --target wasm32-unknown-unknown

FROM ubuntu:24.04
ARG STDB_VERSION=2.10.2
ARG TARGETARCH
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl openssl \
 && rm -rf /var/lib/apt/lists/*
RUN case "${TARGETARCH:-amd64}" in arm64) arch=aarch64 ;; *) arch=x86_64 ;; esac \
 && curl -fsSL "https://github.com/clockworklabs/SpacetimeDB/releases/download/v${STDB_VERSION}/spacetime-${arch}-unknown-linux-gnu.tar.gz" \
    | tar xz -C /usr/local/bin spacetimedb-standalone spacetimedb-cli
COPY --from=module /src/module/target/wasm32-unknown-unknown/release/rose_stdb_module.wasm /opt/rose/rose_stdb_module.wasm
COPY server/entrypoint.sh /opt/rose/entrypoint.sh
# Strip Windows line endings in case Git converted them on checkout.
RUN sed -i "s/\r$//" /opt/rose/entrypoint.sh && chmod +x /opt/rose/entrypoint.sh

# Everything that must survive a restart: the world, the identity keys and the admin login.
VOLUME /data
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=5s CMD curl -fs http://127.0.0.1:3000/v1/ping || exit 1
ENTRYPOINT ["/opt/rose/entrypoint.sh"]
