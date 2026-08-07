#!/usr/bin/env bash
set -euo pipefail

key="${1:?metadata key is required}"
project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
value="$(grep -m1 "\"$key\"" "$project_root/package.json" | sed 's/.*: *"//; s/".*//')"

if [[ -z "$value" ]]; then
  echo "package.json is missing $key" >&2
  exit 1
fi

echo "$value"
