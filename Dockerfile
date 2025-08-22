# Builder
FROM rust:1.88.0-alpine AS builder

# Install dependencies for openssl-sys and linking
RUN apk update && apk add --no-cache \
    pkgconf \
    openssl-dev \
    musl-dev \
    build-base \
    perl \
    gcc \
    g++

WORKDIR /app

# Copy Cargo.toml and Cargo.lock first to leverage Docker cache
COPY Cargo.toml Cargo.lock ./

# Build dependencies first to cache them
RUN mkdir src && \
    echo "fn main() {println!(\"Hello, world!\");}" > src/mcp && \
    cargo build --release && \
    rm -rf src/mcp

# Copy source code
COPY src ./src


# Build the release binary
RUN cargo build --release

# Create a minimal runtime image
FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/mcp ./

RUN chmod +x /app/mcp


# Command to run the application
CMD ["/app/mcp"]