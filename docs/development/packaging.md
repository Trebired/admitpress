# Packaging

## Layout

```
packaging/fedora/    desktop entry, icon, AppStream metadata
packaging/rpm/       the spec, templated on name and version
packaging/deb/       the control template
packaging/linux/     the build scripts
```

`packaging/linux/meta.sh` reads the name and version from `package.json`, so no
script hardcodes either.

## Building locally

```sh
cargo build -p ticket-kde --release
./packaging/linux/rpm.sh dist
./packaging/linux/deb.sh dist       # needs dpkg-deb
./packaging/linux/appimage.sh dist  # downloads linuxdeploy on first use
```

Each takes an output directory, defaulting to `dist/`, and resolves it to an
absolute path before doing anything, because they all work inside a temporary
directory.

## What each package installs

| Path | From |
| --- | --- |
| `/usr/bin/admitpress` | The release binary |
| `/usr/share/applications/app.ticket.native.desktop` | `packaging/fedora` |
| `/usr/share/icons/hicolor/scalable/apps/app.ticket.native.svg` | `packaging/fedora` |
| `/usr/share/metainfo/app.ticket.native.metainfo.xml` | `packaging/fedora` |
| `/usr/share/licenses/admitpress/LICENSE` | The repository root |

Nothing else. No JavaScript, no QML files, no Bun.

## Dependencies

The RPM and DEB declare Qt 6, Kirigami, the QQC2 desktop style, `cups-client`,
librsvg, poppler, polkit and the Breeze icons. The DEB additionally depends on
`pkexec`, which Debian ships separately from `polkitd`, and on
`kf6-breeze-icon-theme`, because `breeze-icon-theme` is a transitional package
there.

Check a package name against the real archive before adding it:

```sh
podman run --rm debian:trixie sh -c 'apt-get update -qq && apt-cache show <name>'
dnf list --available <name>
```

## The AppImage

`appimage.sh` stages the same tree, then uses `linuxdeploy` with its Qt plugin.
On top of what the plugin finds it adds:

- `org/kde/kirigami` and `org/kde/desktop` QML modules, because the application
  QML is compiled into the binary and the plugin cannot scan it for imports
- The Kirigami platform plugin from `plugins/kf6/kirigami`
- The Qt SVG image and icon engine plugins, without which every icon and the
  logo fail to decode
- `libharfbuzz`, `libICE` and `libSM`, which linuxdeploy excludes but the
  application needs
- The Breeze and Adwaita icon themes
- An `apprun-hooks` script setting the QML import path and `XDG_DATA_DIRS`

The result runs on a system with no Qt or KDE packages at all. Verify that
claim the same way it was verified originally:

```sh
podman run --rm -v "$PWD/dist":/out:z ubuntu:24.04 bash -c '
  apt-get update -qq && apt-get install -y -qq --no-install-recommends \
    libegl1 libgl1 libopengl0 libxkbcommon0 fontconfig xvfb
  APPIMAGE_EXTRACT_AND_RUN=1 xvfb-run -a /out/*.AppImage'
```

Graphics drivers, fonts and CUPS always come from the host, by design.

### Known limitation

The bundled Kirigami platform plugin is not picked up, so the AppImage falls
back to Kirigami's basic platform. It runs and renders correctly but does not
follow the desktop style. RPM and DEB users are unaffected.
