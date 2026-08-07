#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
outdir="${1:-$project_root/dist}"
mkdir -p "$outdir"
outdir="$(cd "$outdir" && pwd)"
meta="$project_root/packaging/linux/meta.sh"
version="$("$meta" version)"
name="$("$meta" binaryName)"
workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

"$project_root/packaging/linux/stage.sh" "$workdir/stage"
mkdir -p "$outdir"

rpmbuild \
  --define "_topdir $workdir/rpmbuild" \
  --define "app_name $name" \
  --define "app_version $version" \
  --define "_stagedir $workdir/stage" \
  --define "_build_id_links none" \
  -bb "$project_root/packaging/rpm/application.spec"

find "$workdir/rpmbuild/RPMS" -name '*.rpm' -exec cp {} "$outdir/" \;
ls -1 "$outdir"/*.rpm
