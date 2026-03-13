# Multi-stage build for optimal image size
# Stage 1: Build the Rust binary
FROM rust:1.85-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests first for dependency caching
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to pre-build and cache dependencies.
# This layer is cached as long as Cargo.toml/Cargo.lock don't change.
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy real source code
COPY src ./src

# Touch main.rs so cargo knows it changed (the dummy was already compiled)
RUN touch src/main.rs

# Build the actual application
RUN cargo build --release && \
    strip target/release/idrac_fan_controller

# Stage 2: Minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
# - ipmitool: required for IPMI communication
# - ca-certificates: for TLS if needed
# - procps: provides pgrep for healthcheck
RUN apt-get update && \
    apt-get install -y --no-install-recommends ipmitool ca-certificates procps && \
    rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /build/target/release/idrac_fan_controller /usr/local/bin/

# Set default environment variables
ENV IDRAC_HOST=local \
    IDRAC_USERNAME=root \
    IDRAC_PASSWORD=calvin \
    MIN_FAN_SPEED=5 \
    MAX_FAN_SPEED=100 \
    BASE_TEMP=40 \
    CRITICAL_TEMP=70 \
    CURVE_STEEPNESS=0.15 \
    CHECK_INTERVAL=60 \
    TEMP_SMOOTHING_WINDOW=3 \
    MIN_CHANGE_INTERVAL=60 \
    EMERGENCY_TEMP_DELTA=10 \
    HYSTERESIS_PERCENT=5 \
    DISABLE_THIRD_PARTY_PCIE_CARD_DELL_DEFAULT_COOLING_RESPONSE=false \
    KEEP_THIRD_PARTY_PCIE_CARD_COOLING_RESPONSE_STATE_ON_EXIT=false \
    RUST_LOG=info

# Health check
HEALTHCHECK --interval=60s --timeout=10s --start-period=10s --retries=3 \
    CMD pgrep -x idrac_fan_controller || exit 1

ENTRYPOINT ["/usr/local/bin/idrac_fan_controller"]
