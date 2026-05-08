#!/bin/bash
set -e

REPO="ShulhaOleh/mkit"
ARCH=$(uname -m)

case "$ARCH" in
    x86_64) BINARY="mkit-x86_64-linux" ;;
    *) echo "unsupported architecture: $ARCH"; exit 1 ;;
esac

curl -fsSL "https://github.com/$REPO/releases/download/latest/$BINARY" \
    -o /usr/local/bin/mkit
chmod +x /usr/local/bin/mkit

echo "mkit installed to /usr/local/bin/mkit"
