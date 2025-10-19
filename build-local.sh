#!/bin/bash

# Build script for chatbox-router
# This script builds the project for the current platform

set -e

echo "Building chatbox-router..."

# Get package name from Cargo.toml
temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]")
package_name=${temp#name=}

echo "Package name: $package_name"

# Build for current platform
cargo build --release

echo "Build completed successfully!"
echo "Binary location: target/release/$package_name"

# Create a copy with platform suffix
cp target/release/$package_name ${package_name}-$(uname -m)

echo "Created copy: ${package_name}-$(uname -m)"