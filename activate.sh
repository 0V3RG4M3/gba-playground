#!/usr/bin/env bash
# Source .env and add MGBA_PATH directory to PATH
if [ -f .env ]; then
    source .env
    export PATH="$(dirname "$MGBA_PATH"):$PATH"
    echo "Added $(dirname "$MGBA_PATH") to PATH."
else
    echo ".env file not found."
fi
