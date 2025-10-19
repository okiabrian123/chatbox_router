#!/bin/bash

# Run script for chatbox-router
# This script runs the application with default or specified parameters

set -e

# Get package name from Cargo.toml
temp=$(cat ./Cargo.toml | grep -E "^name\s*=" | awk -F "=" "{print $2}" | tr -d "\"" | tr -d "[:space:]")
package_name=${temp#name=}

# Default values
DEFAULT_IP="127.0.0.1"
DEFAULT_PORT="8080"

# Parse command line arguments
IP=${1:-$DEFAULT_IP}
PORT=${2:-$DEFAULT_PORT}

echo "Starting $package_name..."
echo "IP: $IP"
echo "Port: $PORT"

# Check if binary exists
if [ ! -f "target/release/$package_name" ]; then
    echo "Binary not found. Building first..."
    ./build-local.sh
fi

# Check if config file exists
if [ ! -f ".config/px.toml" ]; then
    echo "Config file not found: .config/px.toml"
    echo "Please create the config file before running."
    exit 1
fi

# Run the application
echo "Running: target/release/$package_name --ip $IP --port $PORT"
./target/release/$package_name --ip $IP --port $PORT