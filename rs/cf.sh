#!/bin/bash
new_file=$1

if [ -z "$new_file" ]; then
	echo "Error: No argument provided. Please specify an argument."
	exit 1
fi
cp src/main.rs "src/bin/${new_file}.rs"
echo "create file: src/bin/${new_file}.rs"

