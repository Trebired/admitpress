# Setup

## Dependencies

```sh
./scripts/install-dependencies.sh
```

Detects Fedora, Debian/Ubuntu and Arch families and installs the Qt 6, KDE
Frameworks 6, CUPS, librsvg, poppler and polkit packages the build and the
runtime need.

Bun and a Rust toolchain are separate. Install Bun from `bun.sh`, and Rust from
`rustup.rs`; `rust-toolchain.toml` pins the stable channel, which only rustup
honours.

On GNOME the KDE libraries install as ordinary libraries and do not pull in a
Plasma session.

## Build and run

```sh
bun i
bun run dev
```

`bun run dev` builds and runs the debug binary against the repository's own
`data/` directory. It writes nothing outside the checkout; a verify check
enforces that.

Because it uses the repository's data directory, a development run and an
installed copy have separate databases.

## Direct cargo

The Bun scripts wrap cargo. Both work:

```sh
cargo build -p ticket-kde
cargo test -p ticket-core
cargo run -p ticket-kde --bin admitpress
```

`QMAKE` must point at `qmake6` if it is not on the path. The build links
against Qt 6 and KF6, so their development packages must be installed.

## Editor support

`qmllint` runs as part of `bun run verify`. Warnings about `TicketBackend` not
being found appear when the QML module has not been built yet; they disappear
after a build.
