#!/bin/bash
set -euo pipefail

echo "=== BMS pub/sub ==="
install/bin/bms_test

echo ""
echo "=== RPC ==="
install/bin/rpc_test_server &>/dev/null &
sleep 5
install/bin/rpc_test_client
kill %1 2>/dev/null; wait 2>/dev/null
