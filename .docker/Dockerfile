FROM rust:1.91.1-slim as build

RUN rustup target add x86_64-unknown-linux-musl && \
    rustup component add rustfmt clippy && \
    apt update && \
    apt install -y musl-tools musl-dev gcc-x86-64-linux-gnu && \
    update-ca-certificates

ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=x86_64-linux-gnu-gcc

WORKDIR /app

COPY Cargo.toml Cargo.lock rustfmt.toml ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm -rf src target


COPY ./src ./src

RUN cargo fmt && cargo clippy && cargo test

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM alpine:3.22

RUN apk add --no-cache wget

WORKDIR /app

RUN addgroup -g 10001 kv && \
    adduser \
    -D \
    -h /nonexistent \
    -s /sbin/nologin \
    -u 10001 \
    -G kv \
    kv

USER kv:kv

COPY --from=build --chown=kv:kv /app/target/x86_64-unknown-linux-musl/release/kv_service ./kv_service

EXPOSE 3000

HEALTHCHECK --interval=1s --timeout=1s --start-period=1s --retries=3 \
  CMD wget --no-verbose --tries=1 http://localhost:3000/healthz || exit 1

ENTRYPOINT ["./kv_service"]