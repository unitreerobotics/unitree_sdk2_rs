#!/bin/bash
set -euo pipefail
DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

docker run --rm --platform linux/arm64 \
    -v "$DIR/install:/app" -w /app/bin \
    sport_console:latest \
    bash -c '
        echo "=== BMS pub/sub ==="
        ./bms_test

        echo ""
        echo "=== RPC ==="
        ./rpc_test_server &>/dev/null & sleep 5
        ./rpc_test_client
        kill %1 2>/dev/null; wait 2>/dev/null
    '
