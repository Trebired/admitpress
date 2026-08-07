# Architecture

## Crates

| Crate | Owns |
| --- | --- |
| `ticket-core` | Domain, SQLite storage, rendering, printing, backups, currency, release checks. No Qt. |
| `ticket-kde` | The cxx-qt bridge, the QML interface, packaging metadata. No domain logic. |

`ticket-core` never depends on Qt, so its tests run without a display. All 56
tests live there.

## Layers inside ticket-core

```
service/   operations the interface calls, one module per area
storage/   every SQL statement, the only place holding a Connection
models.rs  the serialisable shapes that cross the bridge
renderer/  ticket layout to SVG
```

`Storage.connection` is private. Anything needing SQL lives in `storage/` as an
`impl Storage` block; `service/` calls those methods and owns no SQL.

## How QML reaches Rust

`crates/ticket-kde/src/backend.rs` is a `#[cxx_qt::bridge]`. It declares one
QObject, `TicketBackend`, exposing:

- `#[qproperty]` for `state_json`, `status_message` and `async_response_json`
- `#[qinvokable]` for every operation QML can call

Implementations live in `src/backend/*.rs`, grouped by area. Every invokable
returns a JSON string shaped `{"ok": true, "data": ...}` or
`{"ok": false, "error": "..."}`. QML unwraps it with `root.parseResponse()` or
`root.acceptResponse()`.

### Synchronous and asynchronous

Fast calls return their JSON directly.

Slow calls take an `_async` suffix, return nothing, and do the work on a
`std::thread` using `with_default_service`, which opens its own service and
connection so it never contends with the interface. The result is pushed back
by setting `async_response_json`, and `routeAsyncResponse` in `app/main.qml`
dispatches it by operation name.

Anything that touches the network or the printer is asynchronous. Discovering
CUPS queues on the interface thread once cost a full second on every page
change.

## The QML tree

```
app/         the window, routing, async dispatch
pages/       home, editor, print
components/  shared widgets and singletons
dialogs/     every modal
editor/      editor internals
print/       print page internals
sidebars/    the global drawer
assets/      logo and icon SVGs
```

QML is compiled into the binary by `qmlcachegen` and Qt resources. There are no
loose QML files at runtime; `build.rs` discovers every `.qml`, `qmldir` and
asset and generates the resource file.

### Singletons

| Singleton | Holds |
| --- | --- |
| `LocaleText` | `tr()`, translation lookup through the backend |
| `TextFormat` | Locale dates and times, ticket counts, number padding |
| `MoneyFormat` | Money formatting with grouped thousands |
| `AppIcons` | Paired Breeze and Adwaita icon names, chosen at runtime |
| `FieldClipboard` | Editor fields copied between ticket instances |
| `PageColumn` | Kirigami column properties applied only where supported |

## Tooling

TypeScript under `tools/` runs the build, the tests and the release gate. None
of it ships. `package.json` has no runtime dependencies and is private.

See [Commands](commands.md) for what each script does.
