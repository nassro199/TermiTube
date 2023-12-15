#!/bin/bash

set -e  # Exit on error
set -o pipefail  # Fail on the first command in a pipeline that fails

# Function to display an error message and exit
die() {
    echo -e "\033[1;31mError: $1\033[0m" >&2
    exit 1
}

# Function to display an informative message
info() {
    echo -e "\033[1;34m$1\033[0m"
}

# Check for the presence of 'cargo'
command -v cargo >/dev/null 2>&1 || die "Rust is not installed or 'cargo' is not in the PATH. Please install Rust before running this script."

info "Building TermiTube..."

# Build the Rust application in release mode
cargo build --release || die "Rust build failed. Please check the build output for details."

info "Copying TermiTube to /usr/local/bin..."

# Copy the binary to /usr/local/bin with the name "TermiTube"
sudo cp target/release/TermiTube /usr/local/bin/TermiTube || die "Failed to copy TermiTube to /usr/local/bin. Make sure you have the necessary permissions."

info "TermiTube has been installed successfully."

exit 0
