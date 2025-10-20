# Multi-stage build for production
# Build stage using rust:latest
FROM rust:latest AS builder

# Install cross-compilation tools and required packages
RUN apt-get update && apt-get install -y \
    gcc-x86-64-linux-gnu \
    g++-x86-64-linux-gnu \
    libssl-dev \
    pkg-config \
    curl \
    ca-certificates \
    default-libmysqlclient-dev \
    binutils-x86-64-linux-gnu \
    libdbus-1-dev \
    && rm -rf /var/lib/apt/lists/*

# Create symbolic links for Rust cross-compilation
RUN ln -sf /usr/bin/x86_64-linux-gnu-gcc /usr/bin/x86_64-unknown-linux-gnu-gcc && \
    ln -sf /usr/bin/x86_64-linux-gnu-g++ /usr/bin/x86_64-unknown-linux-gnu-g++

# Set up cargo config for cross-compilation
RUN mkdir -p /root/.cargo && \
    echo '[target.x86_64-unknown-linux-gnu]' > /root/.cargo/config.toml && \
    echo 'linker = "x86_64-linux-gnu-gcc"' >> /root/.cargo/config.toml

# Install Rust target
RUN rustup target add x86_64-unknown-linux-gnu

# Set working directory
WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --target=x86_64-unknown-linux-gnu && rm -rf src

# Copy source code
COPY src ./src
COPY .config ./config

# Build the application
RUN cargo build --release --target=x86_64-unknown-linux-gnu

# Runtime stage using debian:bullseye
FROM debian:bullseye

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create app user
RUN useradd -r -s /bin/false appuser

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/chatbox-router ./chatbox-router

# Copy config
COPY --from=builder /app/config ./.config

# Create necessary directories
RUN mkdir -p /var/log/program

# Set permissions
RUN chown -R appuser:appuser /app
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the application
CMD ["./chatbox-router"]
