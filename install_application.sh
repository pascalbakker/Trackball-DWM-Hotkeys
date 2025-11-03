#!/usr/bin/env bash
set -e
BINARY_PATH="./target/release/hotkeys"
DEST_DIR="/usr/local/bin"
if [ ! -f "$BINARY_PATH" ]; then
    echo "Error: binary not found at $BINARY_PATH"
    exit 1
fi

sudo mkdir -p "$DEST_DIR"
sudo cp "$BINARY_PATH" "$DEST_DIR/"
sudo chmod +x "$DEST_DIR/hotkeys"
echo "Installed hotkeys to $DEST_DIR/hotkeys"
echo "You can now run it from anywhere by typing: hotkeys"
