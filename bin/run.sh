#!/usr/bin/env sh
set -eu

slug="$1"
solution_dir="$2"
output_dir="$3"

/opt/analyzer/bin/rust-analyzer "$slug" "$solution_dir" "$output_dir" 
