#!/bin/bash

echo "Starting Viya PSP DouChong Bang Application (Backend + Frontend on port 2992)..."

# Check if Rust is installed
echo "Checking Rust installation..."
if ! command -v rustc &> /dev/null; then
    echo "Error: Rust toolchain not found."
    exit 1
else
    echo "Rust is installed: $(rustc --version)"
fi

# Check if Node.js is installed
echo "Checking Node.js installation..."
if ! command -v npm &> /dev/null; then
    echo "Error: npm not found."
    exit 1
else
    echo "npm version: $(npm --version)"
fi

# Build frontend first
echo "Building frontend..."
cd frontend
echo "Current directory: $(pwd)"
npm install
if [ $? -ne 0 ]; then
    echo "Warning: npm install failed, continuing anyway..."
else
    echo "npm install completed successfully."
fi
npm run build
if [ $? -ne 0 ]; then
    echo "Error: Failed to build frontend"
    exit 1
else
    echo "Frontend build completed successfully."
fi

# Return to main directory
cd ..
echo "Returned to main directory: $(pwd)"

# Build backend first to ensure it compiles
echo "Building backend..."
cd rust_backend
echo "Current directory: $(pwd)"
cargo build
if [ $? -ne 0 ]; then
    echo "Error: Failed to build backend"
    exit 1
else
    echo "Backend build completed successfully."
fi

# Start the combined backend+frontend server
echo "Starting combined server on port 2992..."
echo "Note: Server is running. Press Ctrl+C to stop the server."
exec cargo run