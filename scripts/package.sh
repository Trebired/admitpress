#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
output="${1:-$project_root/../application-source-0.1.0.tar.gz}"
project_name="$(basename "$project_root")"
parent_dir="$(dirname "$project_root")"

rm -f "$output"
tar \
  --exclude="$project_name/.git" \
  --exclude="$project_name/target" \
  --exclude="$project_name/data/application.sqlite" \
  --exclude="$project_name/data/application.sqlite-shm" \
  --exclude="$project_name/data/application.sqlite-wal" \
  --exclude="$project_name/data/development-prints" \
  --exclude="$project_name/data/print-jobs" \
  -czf "$output" \
  -C "$parent_dir" \
  "$project_name"

sha256sum "$output"
