# Multi-stage build for rshioaji manylinux_x86_64 with Python 3.12 support
FROM rust:1.75-slim as rust-builder

# Install system dependencies for Python 3.12 and static linking
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    python3.12-dev \
    python3.12-venv \
    python3-pip \
    binutils \
    musl-tools \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Ensure Python 3.12 is available
RUN update-alternatives --install /usr/bin/python3 python3 /usr/bin/python3.12 1

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock build.rs ./

# Copy source code
COPY src/ ./src/
COPY lib/ ./lib/
COPY examples/ ./examples/
COPY tests/ ./tests/

# Set environment to use manylinux_x86_64 with Python 3.12 support
ENV RSHIOAJI_PLATFORM=manylinux_x86_64
ENV CARGO_FEATURE_STATIC_LINK=1
ENV PYTHON_VERSION=3.12
ENV PYO3_PYTHON=python3.12

# Add target for x86_64 Linux and build with Python 3.12 support
RUN rustup target add x86_64-unknown-linux-gnu
RUN cargo build --release --target x86_64-unknown-linux-gnu --features static-link

# Verify the binary is built for x86_64 and check dependencies
RUN file /app/target/x86_64-unknown-linux-gnu/release/rshioaji-cli
RUN ldd /app/target/x86_64-unknown-linux-gnu/release/rshioaji-cli || echo "Static binary confirmed"

# Final runtime image - minimal base
FROM debian:bookworm-slim

# Install minimal runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd -r rshioaji && useradd -r -g rshioaji rshioaji

# Set working directory
WORKDIR /app

# Copy the statically linked binary for x86_64
COPY --from=rust-builder /app/target/x86_64-unknown-linux-gnu/release/rshioaji-cli /usr/local/bin/rshioaji

# Copy the shared library if needed
COPY --from=rust-builder /app/target/x86_64-unknown-linux-gnu/release/librshioaji.so /usr/local/lib/ || true

# Copy examples if needed (no lib directory needed for static build)
COPY --from=rust-builder /app/examples/ ./examples/

# Set minimal environment variables for Python 3.12
ENV RSHIOAJI_PLATFORM=manylinux_x86_64
ENV PYTHON_VERSION=3.12
ENV LD_LIBRARY_PATH=/usr/local/lib:$LD_LIBRARY_PATH

# Change ownership to non-root user
RUN chown -R rshioaji:rshioaji /app && \
    chmod +x /usr/local/bin/rshioaji

# Switch to non-root user
USER rshioaji

# Expose any ports if needed (adjust as necessary)
# EXPOSE 8080

# Set the default command
CMD ["rshioaji"]