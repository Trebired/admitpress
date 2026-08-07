#!/usr/bin/env bash
set -euo pipefail

if [[ -r /etc/os-release ]]; then
  . /etc/os-release
fi

fedora_packages=(
  cmake gcc-c++ mold
  extra-cmake-modules kf6-kirigami-devel kf6-qqc2-desktop-style
  qt6-qtbase-devel qt6-qtdeclarative-devel qt6-qtwayland-devel
  cups cups-client librsvg2-tools poppler-utils inkscape
  xdg-desktop-portal-gtk polkit breeze-icon-theme adwaita-icon-theme
)

debian_packages=(
  cmake g++ mold
  extra-cmake-modules libkirigami-dev qml6-module-org-kde-kirigami
  libkf6qqc2desktopstyle-dev qml6-module-org-kde-desktop
  qml6-module-qtquick-controls qml6-module-qtquick-dialogs
  qt6-base-dev qt6-declarative-dev qt6-tools-dev qt6-wayland qt6-svg-plugins
  cups cups-client librsvg2-bin poppler-utils inkscape
  xdg-desktop-portal-gtk polkitd pkexec kf6-breeze-icon-theme adwaita-icon-theme
)

arch_packages=(
  cmake gcc mold
  extra-cmake-modules kirigami qqc2-desktop-style
  qt6-base qt6-declarative qt6-wayland
  cups libcups librsvg poppler inkscape
  xdg-desktop-portal-gtk polkit breeze-icons adwaita-icon-theme
)

detect_family() {
  local id="${ID:-}" like="${ID_LIKE:-}"
  case "$id" in
    fedora|rhel|centos|rocky|almalinux) echo fedora; return;;
    debian|ubuntu|linuxmint|pop|elementary) echo debian; return;;
    arch|manjaro|endeavouros|cachyos) echo arch; return;;
  esac
  case "$like" in
    *fedora*|*rhel*) echo fedora;;
    *debian*|*ubuntu*) echo debian;;
    *arch*) echo arch;;
    *) echo unknown;;
  esac
}

install_batch() {
  local family="$1"
  shift
  case "$family" in
    fedora) sudo dnf install -y "$@";;
    debian) sudo apt-get install -y "$@";;
    arch) sudo pacman -S --needed --noconfirm "$@";;
  esac
}

install_tolerant() {
  local family="$1"
  shift
  if install_batch "$family" "$@" 2>/dev/null; then
    return 0
  fi
  echo "Batch install failed, retrying package by package." >&2
  local skipped=()
  for package in "$@"; do
    install_batch "$family" "$package" >/dev/null 2>&1 || skipped+=("$package")
  done
  if [ -n "${skipped[*]:-}" ]; then
    echo "Not available on this distribution: ${skipped[*]}" >&2
    echo "Install the equivalents by hand before building." >&2
  fi
}

family="$(detect_family)"
case "$family" in
  fedora) packages=("${fedora_packages[@]}");;
  debian) packages=("${debian_packages[@]}"); sudo apt-get update;;
  arch) packages=("${arch_packages[@]}");;
  *)
    echo "Unsupported distribution: ${PRETTY_NAME:-unknown Linux}." >&2
    echo "Install Qt 6, KF6 Kirigami, KF6 QQC2 desktop style, CUPS, polkit," >&2
    echo "xdg-desktop-portal-gtk, breeze-icon-theme and Rust, then build with cargo." >&2
    exit 1
    ;;
esac

if ! command -v cargo >/dev/null 2>&1 || ! command -v rustc >/dev/null 2>&1; then
  case "$family" in
    fedora) packages=(rust cargo "${packages[@]}");;
    debian) packages=(rustc cargo "${packages[@]}");;
    arch) packages=(rust "${packages[@]}");;
  esac
fi

install_tolerant "$family" "${packages[@]}"

if ! command -v qmake6 >/dev/null 2>&1; then
  echo "qmake6 is still missing; check the Qt 6 base development package." >&2
  exit 1
fi

qmake6 -query QT_VERSION

if ! command -v pkexec >/dev/null 2>&1; then
  echo "pkexec is missing, so the printer repair cannot ask for administrator rights." >&2
fi

if ! command -v bun >/dev/null 2>&1; then
  echo "Bun is required for project scripts. Install Bun before running the app." >&2
fi

echo "Dependencies for ${PRETTY_NAME:-$family} are installed. Run bun run dev to start the app."
