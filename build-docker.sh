#!/bin/bash

# Docker build script for Linux x86_64
# This script builds the project using Docker for consistent cross-compilation

set -e

echo "Building chatbox-router for Linux x86_64 using Docker..."

# Get package name from Cargo.toml
temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]")
package_name=${temp#name=}

echo "Package name: $package_name"

# Create Dockerfile for building if it doesn't exist
if [ ! -f "Dockerfile.build" ]; then
    cat > Dockerfile.build << 'EOF'
FROM rust:1.75-slim

# Install build dependencies
RUN apt-get update && apt-get install -y \
    gcc \
    g++ \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Copy source code
COPY src ./src
COPY .config ./config

# Build the application
RUN cargo build --release

# Copy the binary to a known location
RUN cp target/release/chatbox-proxy_handler /app/chatbox-proxy_handler-x86_64
EOF
fi

# Build Docker image
docker build -f Dockerfile.build -t chatbox-router-builder .

# Create container and copy binary
docker create --name chatbox-router-build chatbox-router-builder
docker cp chatbox-router-build:/app/chatbox-proxy_handler-x86_64 ./
docker rm chatbox-router-build

# Clean up Docker image
docker rmi chatbox-router-builder

echo "Build completed successfully!"
echo "Binary created: chatbox-proxy_handler-x86_64"