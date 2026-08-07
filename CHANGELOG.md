# Changelog

All notable changes to AdmitPress will be documented here.

This project follows semantic versioning once published.

## Unreleased

- Changed `bun run dev` so it never writes outside the repository. It previously installed a desktop entry, an icon and AppStream metadata into the user data directory and refreshed the desktop caches.
- Added a Development sandbox check to `bun run verify` that fails when the native tooling reaches outside the repository.
- Fixed the development launcher shadowing an installed copy. Running the application from the desktop started a build in the source tree instead of the installed binary, because the entry in the user data directory takes precedence over the packaged one.

## 0.1.0

- Added the ticket editor with static text, image, price, date, number, group name, and group item fields across a classic and a group layout.
- Added per-price daily counters, counter adjustments, and sales that print or record without printing.
- Added named sales and group tickets for any sale of two or more tickets.
- Added the sales overview grouped by month, with day and month detail dialogs and spreadsheet export.
- Added today's activity, logging prints, records, failures, and counter corrections, with a search field and a kind filter.
- Added a main currency setting defaulting to the operating system locale and falling back to the US dollar, per sale currency selection, and exchange rates fetched online and cached for six hours.
- Added automatic backups of the database and ticket artwork every three hours, with a startup catch-up, automatic thinning, and a Backups item in the sidebar.
- Added the sidebar footer showing the organization, the current version, and links to the organization and application sites.
- Added a dialog that opens once per newly detected release, at startup or mid-session, with the link to install it from.
- Added a display-only update check that reads the latest release tag every three hours and never downloads or installs anything.
- Added copy and paste in the editor, including pasting fields into another ticket.
- Added GNOME support: paired Breeze and Adwaita icon names resolved at runtime, a resizable window, desktop and AppStream metadata, and setup for Fedora, Debian, and Arch families.
- Added RPM, DEB and AppImage packages built and published by the release workflow on a version tag. The AppImage bundles Qt 6, Kirigami, the QQC2 desktop style, the Qt SVG plugins and the Breeze and Adwaita icon themes, so it runs with no Qt or KDE packages installed.
- Fixed the application failing to start against Kirigami older than 6.16, whose ColumnView has no minimum, preferred or interactive resize properties. Column sizing is now applied only where the running Kirigami supports it.
- Changed the default data directory to `$XDG_DATA_HOME/admitpress` so an installed build no longer writes to the launcher working directory.
- Changed dates and times to follow the operating system locale everywhere, with a four digit year.
- Changed printer repair to report a clear error when pkexec is missing instead of hanging on a password prompt that never appears.
- Changed the window icon to follow the light and dark theme at runtime.
- Changed the README into a landing page with the logo, screenshots and package tables.
- Fixed page switching stalling for about a second because CUPS queues were discovered on the user interface thread.
- Fixed editor fields shifting when selected with a single click.
