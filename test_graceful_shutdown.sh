#!/bin/bash
# Test script for graceful shutdown

echo "Starting server..."
RUST_LOG=info cargo run --quiet &
SERVER_PID=$!

# Wait for server to start
sleep 2

echo "Making test request..."
curl -s http://127.0.0.1:3000/health > /dev/null && echo "✓ Server responding"

echo "Sending CTRL+C (SIGINT) to server..."
kill -INT $SERVER_PID

# Wait for graceful shutdown
sleep 1

if ! ps -p $SERVER_PID > /dev/null 2>&1; then
    echo "✓ Server shut down gracefully"
else
    echo "✗ Server still running, forcing kill"
    kill -9 $SERVER_PID
fi
