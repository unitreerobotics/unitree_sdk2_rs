#!/bin/bash
set -euo pipefail
REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

docker run --rm --platform linux/arm64 \
    -v "$REPO_ROOT:/workspace" -w /workspace/unitree_sdk2_rs \
    sport_console:latest \
    bash ./build.sh
