# Multi-stage build for ElectronBeam CLI
FROM rust:1.75-slim as builder

# Install system dependencies needed for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/
COPY examples/ ./examples/

# Build the application in release mode
RUN cargo build --release --bin electron-beam

# Runtime stage - use minimal Alpine image
FROM alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# Create a non-root user
RUN addgroup -g 1001 -S electronbeam && \
    adduser -u 1001 -S electronbeam -G electronbeam

# Create directories for input/output
RUN mkdir -p /app/input /app/output && \
    chown -R electronbeam:electronbeam /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/electron-beam /usr/local/bin/electron-beam

# Make binary executable
RUN chmod +x /usr/local/bin/electron-beam

# Switch to non-root user
USER electronbeam

# Set working directory
WORKDIR /app

# Set default entrypoint
ENTRYPOINT ["electron-beam"]

# Default command shows help
CMD ["--help"]

# Metadata
LABEL org.opencontainers.image.title="ElectronBeam"
LABEL org.opencontainers.image.description="A CLI tool for creating nostalgic CRT-style turn-off animations from PNG images to GIF format"
LABEL org.opencontainers.image.url="https://github.com/aovestdipaperino/electron-beam"
LABEL org.opencontainers.image.source="https://github.com/aovestdipaperino/electron-beam"
LABEL org.opencontainers.image.licenses="MIT"

# Usage examples in comments:
# Build: docker build -t electron-beam .
# Run with help: docker run --rm electron-beam
# Process image: docker run --rm -v $(pwd):/app/input -v $(pwd):/app/output electron-beam -i /app/input/image.png -o /app/output/animation.gif
# Interactive: docker run --rm -it -v $(pwd):/app/input -v $(pwd):/app/output electron-beam -i /app/input/image.png -o /app/output/animation.gif -m cool-down -f 30 -d 100 --verbose
