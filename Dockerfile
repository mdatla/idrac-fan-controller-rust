# Multi-stage build for optimal image size
FROM rust:1.75-slim as builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Copy manifests
COPY Cargo.toml Cargo.lock* ./

# Create a dummy main.rs to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release || true && \
    rm -rf src

# Copy source code
COPY src ./src

# Build the actual application
RUN cargo build --release && \
    strip target/release/idrac_fan_controller

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ipmitool ca-certificates && \
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
