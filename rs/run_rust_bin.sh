#!/bin/bash

BIN_DIR="src/bin"
TARGET=$(find $BIN_DIR -name "*.rs" | fzf)

if [ -z "$TARGET" ]; then
	echo "No target selected."
	exit 1
fi

TARGET_NAME=$(basename -s .rs "$TARGET")
cargo run --bin "$TARGET_NAME"

