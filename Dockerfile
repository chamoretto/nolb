# ===== This image is "light", ready for pulling it from registry - ~7MB image (~18MB uncompressed size)  =====

# ===== Base image with downloaded crates and build dependencies =====
FROM instrumentisto/rust:1.58-alpine as base

# Setup dependencies and Rust
RUN set -eux; \
    apk add --no-cache \
    mold \
    perl \
    make \
    openssl-dev \
    musl-dev; \
    rustup --version; \
    cargo --version; \
    rustc --version;

# Set the working directory to /app
WORKDIR /app

# Copy the Cargo.toml contents into the container at /app
COPY Cargo.toml /app/Cargo.toml

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# creating temp directory in order to initialize and cache cargo registry /tmp/cache/Cargo.toml
RUN mkdir -p /tmp/cache/src/ &&  \
    touch /tmp/cache/src/main.rs && \
	cp /app/Cargo.toml /tmp/cache/Cargo.toml && \
	cargo fetch --manifest-path /tmp/cache/Cargo.toml && \
	rm -rf /tmp/cache;

# Copy actual changed source code
COPY .rustfmt.toml    /app/.rustfmt.toml
COPY src              /app/src


# ===== Build app at separate image =====
FROM base AS builder

# Build the changed sourse code. If none of dependencies changed, we won't download them
RUN mold -run cargo build --release --offline &&  \
    rm -rf /app/target/release/build \
    /app/target/release/deps \
    /app/target/release/examples  \
    /app/target/release/incremental \
    /app/target/release/nolb-cli.exe.d && \
    apk del musl-dev openssl-dev;


# ===== Copy executable file and configs into pure small image =====
FROM alpine:latest

COPY --from=builder /app/target/release/nolb-cli.exe /app/nolb-cli.exe
ENV PATH=/app:$PATH

WORKDIR /app

ENTRYPOINT ["nolb-cli.exe"]
