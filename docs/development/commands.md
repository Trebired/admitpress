# Commands

## Everyday

| Command | Does |
| --- | --- |
| `bun run dev` | Build and run the debug binary against the repository `data/` |
| `bun run build` | Build the release binary |
| `bun run test` | Run the Rust tests |
| `bun run verify` | The full release gate, 14 checks |
| `bun run discipline` | Report Code Discipline violations |
| `bun run discipline:fix` | Apply the fixable violations |
| `bun run typecheck` | `tsc --noEmit` over the tooling |

`dev`, `build`, `test` and `verify` run behind the Code Discipline gate, so a
violation stops them before anything else happens. The `:raw` variants skip the
gate when you need the underlying command alone.

## What verify checks

| Check | Confirms |
| --- | --- |
| Required files | The files a release cannot ship without |
| TOML parsing | Every manifest parses, workspace members resolve |
| SQLite workflow | Migration and the transactional counter path |
| SVG and rasterization | A ticket SVG parses and rasterises to the expected size |
| Source delimiter scan | Brackets and braces balance in every source file |
| Source policy | No em dashes, unfinished markers, banned names or banned time field names |
| Packaging metadata | Desktop entry, icon and AppStream XML validate |
| Shell syntax | Every packaging script parses |
| Development sandbox | The native tooling does not reach outside the repository |
| QML backend contract | Every QML call matches a declared invokable |
| QML module contract | The module URI matches the import and the resource path |
| Cargo check | The workspace compiles |
| Cargo tests | The tests pass |
| QML lint | `qmllint` over the QML tree |

A check that reports SKIP means a tool was missing, not that it passed. Cargo
checks silently skipped for a while because the executable lookup was broken;
treat a SKIP in CI as a failure to investigate.

## Discipline rules worth knowing

- 350 lines per file, 50 per function, 150 characters per line
- No comments in source; they are removed, not reported
- Duplicate function detection
- Structural blank lines and formatting are rewritten by `discipline:fix`

`discipline:fix` also **moves files** whose names share a prefix with a sibling
into a subdirectory, and does not rewrite `qmldir` entries or imports when it
does. See [Pitfalls](pitfalls.md).
