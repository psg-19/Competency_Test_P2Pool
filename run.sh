#!/bin/bash
set -e

# Update apt and install required dependencies for rust
echo "Updating package lists and installing required dependencies..."
sudo apt update && sudo apt install -y build-essential pkg-config libssl-dev

# Start docker-compose 
echo "Starting docker-compose..."
docker-compose up -d

# Wait for the Bitcoin RPC to be available.
echo "Waiting for RPC to be available..."
while true; do
BLOCK_COUNT=$(sudo docker exec -it competency_test_p2pool-main-bitcoin-1 bitcoin-cli -rpcconnect=127.0.0.1 -rpcport=18443 -rpcuser=alice -rpcpassword=password getblockcount 2>/dev/null || true)
    if [[ -n "$BLOCK_COUNT" ]]; then
        echo "RPC is available. Current block count: $BLOCK_COUNT"
        break
    else
        echo "RPC not available yet. Retrying in 5 seconds..."
        sleep 5
    fi
done

# Run the Rust application.
echo "Starting Rust application with 'cargo run'..."
cargo run

# stoping the Docker Compose services.
echo "Rust application finished. Stopping docker-compose services..."
sudo docker-compose down
