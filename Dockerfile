FROM rust:1.40-alpine AS rust-builder
RUN apk add --no-cache musl-dev
ENV RUSTFLAGS="-C target-feature=-crt-static"
RUN cargo install cargo-build-deps
RUN cd /tmp && USER=root cargo new --bin fgo-import
WORKDIR /tmp/fgo-import
COPY Cargo.toml Cargo.lock ./
RUN cargo build-deps --release
COPY src /tmp/fgo-import/src
RUN cargo build --release

FROM alpine
ENV RUST_BACKTRACE=1
RUN apk add --no-cache libgcc
WORKDIR /usr/bin
COPY --from=rust-builder /tmp/fgo-import/target/release/fgo-import .
CMD ["./fgo-import"]