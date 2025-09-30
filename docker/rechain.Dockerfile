# Multi-stage Dockerfile for REChain Node - REChain Network Solutions LLC
# Optimized for security, performance, and maintainability

# === Build Stage ===
FROM rust:1.75-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    cmake \
    libssl-dev \
    pkg-config \
    protobuf-compiler \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml ./
COPY substrate/frame/*/Cargo.toml ./substrate/frame/*/
COPY rechain/Cargo.toml ./rechain/
COPY cumulus/*/Cargo.toml ./cumulus/*/
COPY bridges/*/Cargo.toml ./bridges/*/
COPY scripts/ ./scripts/
COPY .cargo/ ./.cargo/

# Copy source code
COPY substrate/frame/ ./substrate/frame/
COPY rechain/ ./rechain/
COPY cumulus/ ./cumulus/
COPY bridges/ ./bridges/

# Build the application in release mode
RUN cargo build --release --bin rechain

# === Runtime Stage ===
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false -m -d /app rechain

# Set working directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/rechain /usr/local/bin/rechain

# Create data directory
RUN mkdir -p /app/data && chown -R rechain:rechain /app

# Switch to non-root user
USER rechain

# Expose ports
EXPOSE 30333 9933 9944

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=60s --retries=3 \
    CMD curl -f http://localhost:9933/health || exit 1

# Set environment variables
ENV RUST_LOG=info

# Default command
ENTRYPOINT ["/usr/local/bin/rechain"]

# Default arguments (can be overridden)
CMD ["--dev", "--ws-external", "--rpc-external"]