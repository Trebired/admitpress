#!/usr/bin/env bash
set -euo pipefail

project_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
outdir="${1:-$project_root/dist}"
mkdir -p "$outdir"
outdir="$(cd "$outdir" && pwd)"
meta="$project_root/packaging/linux/meta.sh"
version="$("$meta" version)"
name="$("$meta" binaryName)"
tools="${APPIMAGE_TOOL_DIR:-$project_root/target/appimage-tools}"
workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT

export APPIMAGE_EXTRACT_AND_RUN=1

fetch_tool() {
  local file="$tools/$1"
  if [[ ! -x "$file" ]]; then
    mkdir -p "$tools"
    curl -fsSL -o "$file" "$2"
    chmod +x "$file"
  fi
  echo "$file"
}

linuxdeploy="$(fetch_tool linuxdeploy \
  https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage)"
qt_plugin="$(fetch_tool linuxdeploy-plugin-qt \
  https://github.com/linuxdeploy/linuxdeploy-plugin-qt/releases/download/continuous/linuxdeploy-plugin-qt-x86_64.AppImage)"

appdir="$workdir/AppDir"
"$project_root/packaging/linux/stage.sh" "$appdir"

qml_dir="$(qmake6 -query QT_INSTALL_QML)"
target_qml="$appdir/usr/qml"
mkdir -p "$target_qml"

copy_qml_module() {
  local module="$1"
  if [[ -d "$qml_dir/$module" ]]; then
    mkdir -p "$target_qml/$(dirname "$module")"
    cp -a "$qml_dir/$module" "$target_qml/$(dirname "$module")/"
  fi
}

for module in org/kde/kirigami org/kde/desktop; do
  copy_qml_module "$module"
done

plugin_dir="$(qmake6 -query QT_INSTALL_PLUGINS)"
if [[ -d "$plugin_dir/kf6/kirigami" ]]; then
  mkdir -p "$appdir/usr/plugins/kf6"
  cp -a "$plugin_dir/kf6/kirigami" "$appdir/usr/plugins/kf6/"
fi

bundled_lib_dir="$appdir/usr/lib"
mkdir -p "$bundled_lib_dir"
for library in libharfbuzz.so.0 libICE.so.6 libSM.so.6; do
  found="$(find /usr/lib /lib -name "$library" -print -quit 2>/dev/null || true)"
  if [[ -n "$found" ]]; then
    cp -L "$found" "$bundled_lib_dir/"
  fi
done

icon_theme_root="$appdir/usr/share/icons"
mkdir -p "$icon_theme_root"
for theme in breeze breeze-dark Adwaita hicolor; do
  if [[ -d "/usr/share/icons/$theme" ]]; then
    cp -a "/usr/share/icons/$theme" "$icon_theme_root/"
  fi
done

mkdir -p "$appdir/apprun-hooks"
cat > "$appdir/apprun-hooks/$name-env.sh" <<'HOOK'
export QT_QUICK_CONTROLS_STYLE="${QT_QUICK_CONTROLS_STYLE:-org.kde.desktop}"
export QML2_IMPORT_PATH="$APPDIR/usr/qml${QML2_IMPORT_PATH:+:$QML2_IMPORT_PATH}"
export QML_IMPORT_PATH="$APPDIR/usr/qml${QML_IMPORT_PATH:+:$QML_IMPORT_PATH}"
export XDG_DATA_DIRS="$APPDIR/usr/share:${XDG_DATA_DIRS:-/usr/local/share:/usr/share}"
HOOK

export QML_SOURCES_PATHS="$project_root/crates/ticket-kde/src/qml"
qt_plugins="imageformats/libqsvg.so;iconengines/libqsvgicon.so"
qt_plugins="$qt_plugins;wayland-decoration-client;wayland-graphics-integration-client"
qt_plugins="$qt_plugins;wayland-shell-integration;platforms/libqwayland-generic.so"
export EXTRA_QT_MODULES="svg"
export EXTRA_QT_PLUGINS="$qt_plugins;platforms/libqoffscreen.so"
export QMAKE="${QMAKE:-$(command -v qmake6)}"

mkdir -p "$outdir"
cd "$workdir"

"$linuxdeploy" \
  --appdir "$appdir" \
  --plugin qt \
  --desktop-file "$appdir/usr/share/applications/app.ticket.native.desktop" \
  --icon-file "$appdir/usr/share/icons/hicolor/scalable/apps/app.ticket.native.svg" \
  --output appimage

produced="$(find "$workdir" -maxdepth 1 -name '*.AppImage' -print -quit)"
if [[ -z "$produced" ]]; then
  echo "linuxdeploy did not produce an AppImage" >&2
  exit 1
fi

target="$outdir/${name}-${version}-x86_64.AppImage"
mv "$produced" "$target"
chmod +x "$target"
echo "$target"
