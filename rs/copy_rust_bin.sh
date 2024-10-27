#!/bin/bash

BIN_DIR="src/bin"
TARGET=$(find $BIN_DIR -name "*.rs" | fzf)

if [ -z "$TARGET" ]; then
	echo "No target selected."
	exit 1
fi

clip.exe <"$TARGET"

