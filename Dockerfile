# Lightweight multi-stage build for rshioaji
FROM rust:1.75-slim as builder

# Install minimal system dependencies for PyO3 and static linking
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    python3-dev \
    python3.13-dev \
    libpython3.13-dev \
    python3-pip \
    build-essential \
    musl-tools \
    && rm -rf /var/lib/apt/lists/*

# Install shioaji[speed] for PyO3 bridge integration
RUN pip3 install --no-cache-dir "shioaji[speed]"

WORKDIR /app

# Copy source code
COPY Cargo.toml build.rs ./
COPY src/ ./src/
COPY lib/ ./lib/

# Set environment for Linux x86_64 platform
ENV RSHIOAJI_PLATFORM=manylinux_x86_64

# Build with speed and static-link features
RUN cargo build --release --features "static-link,speed"

# Verify binary works
RUN ./target/release/rshioaji-cli --help

# Final runtime image - include Python runtime
FROM debian:bookworm-slim

# Install Python 3.13 runtime and essential dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    python3.13 \
    python3-pip \
    libpython3.13 \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r rshioaji && useradd -r -g rshioaji rshioaji

# Install shioaji[speed] for runtime PyO3 bridge
RUN pip3 install --no-cache-dir "shioaji[speed]"

WORKDIR /app

# Copy the statically linked binary
COPY --from=builder /app/target/release/rshioaji-cli /usr/local/bin/rshioaji

# Set environment
ENV RSHIOAJI_PLATFORM=manylinux_x86_64

# Set non-root user and permissions
RUN chown rshioaji:rshioaji /usr/local/bin/rshioaji && \
    chmod +x /usr/local/bin/rshioaji

USER rshioaji

# Default command
ENTRYPOINT ["rshioaji"]
CMD ["--help"]