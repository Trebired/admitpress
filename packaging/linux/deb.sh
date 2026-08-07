#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
outdir="${1:-$project_root/dist}"
mkdir -p "$outdir"
outdir="$(cd "$outdir" && pwd)"
meta="$project_root/packaging/linux/meta.sh"
version="$("$meta" version)"
name="$("$meta" binaryName)"
architecture="$(dpkg --print-architecture)"
workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

stage="$workdir/stage"
"$project_root/packaging/linux/stage.sh" "$stage"
mkdir -p "$stage/DEBIAN" "$outdir"

sed \
  -e "s/@NAME@/$name/" \
  -e "s/@VERSION@/$version/" \
  -e "s/@ARCHITECTURE@/$architecture/" \
  "$project_root/packaging/deb/control.in" > "$stage/DEBIAN/control"

package="$outdir/${name}_${version}_${architecture}.deb"
dpkg-deb --root-owner-group --build "$stage" "$package"
dpkg-deb --info "$package"
dpkg-deb --contents "$package"
echo "$package"
