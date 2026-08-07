#!/usr/bin/env bash
set -euo pipefail

destdir="${1:?destination directory is required}"
project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
name="$("$project_root/packaging/linux/meta.sh" binaryName)"
target_dir="${CARGO_TARGET_DIR:-$project_root/target}"
binary="$target_dir/release/$name"

if [[ ! -x "$binary" ]]; then
  echo "Release binary is missing. Run cargo build -p ticket-kde --release first." >&2
  exit 1
fi

packaged="$project_root/packaging/fedora"
install -Dm0755 "$binary" "$destdir/usr/bin/$name"
install -Dm0644 "$packaged/app.ticket.native.desktop" \
  "$destdir/usr/share/applications/app.ticket.native.desktop"
install -Dm0644 "$packaged/app.ticket.native.svg" \
  "$destdir/usr/share/icons/hicolor/scalable/apps/app.ticket.native.svg"
install -Dm0644 "$packaged/app.ticket.native.metainfo.xml" \
  "$destdir/usr/share/metainfo/app.ticket.native.metainfo.xml"
install -Dm0644 "$project_root/LICENSE" "$destdir/usr/share/licenses/$name/LICENSE"
