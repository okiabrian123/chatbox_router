#!/bin/bash

# Cross-compilation build script for Linux x86_64
# This script builds the project for Linux x86_64 platform

set -e

echo "Building chatbox-router for Linux x86_64..."

# Check if target is installed
if ! rustup target list --installed | grep -q "x86_64-unknown-linux-gnu"; then
    echo "Installing x86_64-unknown-linux-gnu target..."
    rustup target add x86_64-unknown-linux-gnu
fi

# Get package name from Cargo.toml
temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]")
package_name=${temp#name=}

echo "Package name: $package_name"

# Check if we're on macOS and need cross-compilation setup
if [[ "$OSTYPE" == "darwin"* ]]; then
    # Check if x86_64-linux-gnu-gcc is available
    if ! command -v x86_64-linux-gnu-gcc &> /dev/null; then
        echo "Installing cross-compilation tools..."
        if command -v brew &> /dev/null; then
            brew install x86_64-linux-gnu-gcc
        else
            echo "Please install x86_64-linux-gnu-gcc manually or use Docker"
            exit 1
        fi
    fi
    
    # Create cargo config for cross-compilation
    mkdir -p ~/.cargo
    cat > ~/.cargo/config.toml << 'EOF'
[target.x86_64-unknown-linux-gnu]
linker = "x86_64-linux-gnu-gcc"
EOF
fi

# Build for Linux x86_64
cargo build --release --target=x86_64-unknown-linux-gnu

echo "Build completed successfully!"
echo "Binary location: target/x86_64-unknown-linux-gnu/release/$package_name"

# Create a copy with platform suffix
cp target/x86_64-unknown-linux-gnu/release/$package_name ${package_name}-x86_64

echo "Created copy: ${package_name}-x86_64"