#!/bin/bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

docker run --rm --platform linux/arm64 \
    -v "$(pwd):/workspace/unitree_sdk2_rs" -w /workspace/unitree_sdk2_rs \
    -it sport_console:latest \
    bash 
