# Recipes

## Add a backend operation

1. Implement it on `TicketService` in `crates/ticket-core/src/service/`.
   Anything needing SQL goes in `storage/` as an `impl Storage` method, because
   `Storage.connection` is private.
2. Declare it in the bridge in `crates/ticket-kde/src/backend.rs`:

```rust
#[qinvokable]
fn thing_status(self: &Self) -> QString;
```

3. Implement it in the matching `crates/ticket-kde/src/backend/*.rs`, returning
   `success(json!({...}))` or `failure(error)`.
4. Call it from QML as `backend.thingStatus()` and unwrap with
   `root.parseResponse()`.

For slow work, add an `_async` variant instead: spawn a thread, use
`with_default_service`, finish with `queue_async_response`, and route the reply
in `routeAsyncResponse` in `app/main.qml`.

The QML backend contract check fails if QML calls something the bridge does not
declare, so a typo is caught by `bun run verify`.

## Add a dialog

1. Create `crates/ticket-kde/src/qml/dialogs/<name>.qml` based on `AppDialog`.
   Use `contentSpacing` for spacing and `DialogActions` for the footer so it
   matches every other modal.
2. Register it in `crates/ticket-kde/src/qml/dialogs/qmldir`.
3. Instantiate it in `dialogs/registry.qml` and add an alias there.
4. Add a matching alias in `app/main.qml` so pages can reach it as
   `root.<name>DialogObject`.

Pick a file name that shares no prefix with a sibling, or `discipline:fix` will
move it. See [Pitfalls](pitfalls.md).

## Add a user-visible string

Every string goes through `LocaleText.tr()`:

```qml
text: LocaleText.tr("Back up now")
```

Add the key to `crates/ticket-kde/i18n/app.yml` with both languages:

```yaml
"Back up now":
  en: "Back up now"
  cs: "Zálohovat nyní"
```

The key is the English source string. A concatenation such as
`tr("Version ") + value + tr(" is available")` needs each literal as its own
key, because `tr()` receives the joined result only when the whole argument is
literal.

Never hardcode the product name in QML. Use `root.appDisplayName`, which comes
from `package.json`; the source policy check rejects the literal.

## Add an icon

Icons are paired, because Breeze and Adwaita disagree on almost every name.
Add a role to both maps in `components/icon_theme.qml`:

```qml
backup: "backup",                 // in kdeNames
backup: "drive-harddisk-symbolic" // in gnomeNames
```

Use it as `icon.name: AppIcons.pick("backup")`.

Verify both names exist before relying on them:

```sh
find /usr/share/icons/breeze -name "backup.svg"
find /usr/share/icons/Adwaita -name "drive-harddisk-symbolic.svg"
```

Two names in the shipped set were broken for months because nobody checked.

`ListElement` cannot hold a binding, so models store the role name and the
consumer resolves it with `AppIcons.pick()`.

## Add a verify check

Add a module under `tools/verify/` exporting a function returning a `Check`,
then register it in the list in `tools/verify/report.ts`.

Prove it fails before you trust it. Introduce the fault deliberately, watch the
check report it, then restore.
