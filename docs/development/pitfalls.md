# Pitfalls

Traps in this codebase that have already cost real time. Each is written from a
failure that happened, not from theory.

## QML file names must be unique across the whole tree

`qmlcachegen` derives its generated object names from the base name alone. Two
`.qml` files anywhere in the tree sharing a base name break the link step with
an error that survives `cargo clean`:

```
undefined symbol: ...dialogs_day_detail_qml::qmlData
```

Check before adding a file:

```sh
find crates/ticket-kde/src/qml -name "*.qml" -printf "%f\n" | sort | uniq -d
```

## discipline:fix moves files and does not fix the references

The `redundant-path-segments` rule treats two siblings sharing a name prefix as
a directory waiting to happen. `app_dialog.qml` next to `app_icons.qml` becomes
`app/dialog.qml` and `app/icons.qml`.

It does not rewrite `qmldir` entries or imports, and the move can create a base
name collision, which then fails the link.

Avoid it by never giving a new file a prefix a sibling already uses. After any
`discipline:fix` run, check the duplicate base names and the `qmldir` files.

## discipline:fix mangles shell and regex it does not understand

Two real cases:

- `${#array[@]}` was read as a comment. Everything from `#` to end of line was
  deleted, leaving `if (( ${`, a syntax error, in a script reported as
  successfully fixed.
- A JavaScript regex literal containing `/` and braces broke the formatter's
  brace matching and it reindented the rest of the file.

Prefer `[ -n "${array[*]:-}" ]` over `${#array[@]}`, and string operations over
regex literals in QML. The `Source delimiter scan` and `Shell syntax` checks in
verify catch both, so always run verify after a fix run.

## Kirigami properties are version gated

`ColumnView.minimumWidth`, `preferredWidth` and `interactiveResizeEnabled`
arrived in Kirigami 6.16. Debian stable ships 6.13, where assigning them is a
fatal QML error at startup, not a warning.

Apply anything newer than the oldest supported Kirigami through
`PageColumn.configure()`, which sets a property only when the running Kirigami
has it.

## Icon names differ between Breeze and Adwaita

Modern Adwaita dropped most legacy non-symbolic names, so even `edit-copy`
fails there. Every icon is a pair in `components/icon_theme.qml`.

Confirm both names exist on disk before using them. Two entries in the shipped
set pointed at names Breeze does not have and rendered blank.

## Executable lookup

`findExecutable` uses `command -v`, not `which`. Fedora container images do not
ship `which`, and when the lookup used it every probe failed silently: cargo
was reported as not installed and its checks reported SKIP while CI stayed
green.

Treat a SKIP in a verify report as a failure to investigate.

## Rust in containers

`.cargo/config.toml` pins `clang` as the linker, so any environment building
this project needs clang, not only a C++ compiler.

A container job running as root with `HOME` pointing elsewhere makes rustup
install to a prefix that does not match the path entry. Set `CARGO_HOME` and
`RUSTUP_HOME` explicitly.

## Paths in packaging scripts

Every packaging script works inside a temporary directory. A relative output
path such as `dist` resolves against the wrong directory after the change, and
the final move fails after the package has already been built. Resolve output
directories to absolute paths first.

## Never copy a live SQLite database

The write-ahead log routinely holds more data than the database file. A file
copy loses it. Use `VACUUM INTO`, which is what `snapshot_database` does.

## Development must not write outside the repository

`bun run dev` once installed a desktop entry into `~/.local/share/applications`
pointing at the source tree. That entry takes precedence over a packaged one,
so launching the installed application started a build in the checkout instead.

The `Development sandbox` check in verify now fails if the native tooling
references the home directory or the desktop caches.

## The banned name rules are real

The product name and the organisation name are rejected in source. Read them
from `package.json` through `root.appDisplayName` in QML or the `APP_*`
constants in Rust. Markdown documentation is exempt.
