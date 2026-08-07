# Contributing

AdmitPress is a Trebired product, licensed under AGPL-3.0-only. Contributions are accepted under those same terms.

## Documentation

[`docs/development/`](docs/development/architecture.md) covers the architecture,
the commands, recipes for common changes, packaging, releasing, and the traps in
this codebase worth knowing before you hit them.

## Runtime

Bun is the runtime and package manager. Never use npm. The native side builds with cargo against Qt 6 and KDE Frameworks 6.

## Commands

```sh
bun i
bun run discipline
bun run typecheck
bun run dev
bun run build
bun run test
bun run verify
```

`bun run dev`, `build`, `test`, and `verify` run behind the Code Discipline gate. Run `bun run discipline:fix` to apply the fixable violations.

The Rust crates build and test directly:

```sh
cargo build -p ticket-kde
cargo test -p ticket-core
```

## Rules

- Code Discipline owns file sizes, function sizes, line length, comment removal, formatting, structural spacing, and duplicate detection. Do not add rule-level excludes to pass the gate.
- Every user-facing string goes through `LocaleText.tr()` in QML and needs an `en` and a `cs` entry in `crates/ticket-kde/i18n/app.yml`.
- QML file names must be unique across the whole tree. qmlcachegen derives its generated object names from the base name alone, so two files sharing one base name break the link step with an error that survives `cargo clean`.
- `bun run discipline:fix` moves files that share a name prefix with a sibling into a subdirectory and does not rewrite the `qmldir` entries or imports. Check `find crates/ticket-kde/src/qml -name "*.qml" -printf "%f\n" | sort | uniq -d` after running it.
- Generated output, the local `data/` directory, and discipline reports stay out of Git.

## Verification

`bun run verify` is the release gate. It checks required files, TOML parsing, the SQLite workflow, SVG rasterization, source policy, packaging metadata, shell syntax, the QML to backend invokable contract, the QML module contract, cargo check, cargo tests, and qmllint. All checks must pass.
