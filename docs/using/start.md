# Getting Started

## Install

Download from the [releases page](https://github.com/trebired/admitpress/releases).

| Package | Command |
| --- | --- |
| Fedora and other RPM distributions | `sudo dnf install ./admitpress-<version>.x86_64.rpm` |
| Debian and Ubuntu | `sudo apt install ./admitpress_<version>_amd64.deb` |
| Anything else | `chmod +x admitpress-<version>-x86_64.AppImage` then run it |

The RPM and DEB pull in Qt 6, Kirigami, the QQC2 desktop style, CUPS client
tools, librsvg, poppler and polkit. The AppImage bundles all of those, so it
needs no Qt or KDE packages. Graphics drivers, fonts and CUPS always come from
the host.

## First launch

Start AdmitPress from the application menu, or run `admitpress`.

The application opens on the ticket list, which is empty. Press **Create**,
give the ticket a name and a paper size in centimetres, and confirm. You now
have a ticket with no fields and no prices.

Three things happen on first launch:

- The data directory is created at `~/.local/share/admitpress`
- The main currency is set from your operating system locale, falling back to
  the US dollar if the locale does not name one
- A backup is written, because none exists yet

## Where your data lives

| What | Path |
| --- | --- |
| Database, artwork, print jobs, exports, backups | `~/.local/share/admitpress` |
| Chosen data directory, when you change it | `~/.config/admitpress/data-dir` |

Everything the application owns is inside the data directory. Copy that one
directory to move an installation to another machine.

To use a different directory, open **Settings** and press **Choose...** next to
**Data directory**. The application opens or creates a separate database there
and remembers the choice. Setting `TICKET_APP_DATA_DIR` overrides both.

## Next

- [Designing tickets](tickets.md) to lay out what gets printed
- [Prices and counters](prices.md) before you can sell anything
