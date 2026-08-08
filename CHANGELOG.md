# Changelog

All notable changes to AdmitPress will be documented here.

This project follows semantic versioning once published.

## 0.2.0

- Added a Printer calibration dialog, opened from Settings, separate from the settings form itself. It carries its own printer choice; picking a printer loads that printer's paper size and correction into the dialog without touching anything else. Each printer queue carries the paper size it holds, prints a sheet of edge rulers, corner marks, measured bars and a centre cross at exactly that size, and stores an offset and scale correction measured from that sheet. The correction is applied to printed output only, never to on-screen previews.
- Changed calibration editing so values are only written after Save is pressed, instead of on every number change. Printing the calibration sheet no longer requires saving first, and it now prints on a background thread so the dialog stays responsive instead of freezing until CUPS finishes.
- Fixed the calibration sheet always printing the last saved profile instead of whatever was currently in the dialog's fields. It now renders exactly the page size and correction shown on screen at the moment Print is clicked, saved or not.
- Added printer selection to new tickets and to the editor. A ticket assigned to a printer takes that printer's calibrated paper size, and resizing such a ticket resizes the printer for every ticket using it. Tickets left unassigned keep their own size exactly as before, and 5 x 7 cm remains the fallback.
- Fixed the editor clearing a ticket's printer assignment on every save, which it did by always sending an empty queue.
- Fixed printed tickets not matching the editor preview. The canvas laid text out with Qt's own block metrics while the print renderer placed baselines itself, so line spacing and vertical centring disagreed. The gap grew with every extra line, which is why group tickets, whose ticket list is multi-line, drifted furthest. The canvas now places every line on the baseline the renderer computes, and the two implementations share one documented set of constants pinned by tests.
- Fixed the editor insetting text and images by three pixels that the printer does not inset, and fixed bottom-aligned text sitting on a different baseline from top- and centre-aligned text.
- Added bold and italic to the editor, applied to prints and previews alike.
- Changed font, size, style and alignment edits to apply to every selected field instead of only the primary one.
- Fixed the No fonts found and No currencies found messages rendering at placeholder-page size inside their dropdowns.
- Fixed the editor toolbar spanning only the canvas, so it now runs the full width above the properties sidebar, and restored the missing separator between the undo and copy button groups.
- Added operator and development guides under `docs/`, covering ticket design, prices and counters, selling, sales and export, currency, printing, backups, the architecture, the commands, recipes, packaging, releasing and the pitfalls in this codebase.
- Added a Settings language switcher for English and Czech, defaulting to English.
- Added an Open backups folder button to the Backups dialog.
- Fixed more Czech UI strings, ticket-card padding and text clipping, stale main-currency symbols, and sticky save notifications.
- Fixed printing empty ticket designs by blocking print actions until the editor has printable content.
- Fixed group ticket field geometry staying stale when the ticket dimensions changed from the classic editor tab.
- Changed the Sales tab to use one selection-based More menu with summary and locked-rule-aware reset day actions instead of day/month detail buttons and day deletion.
- Changed the printer debug repair controls to one Repair dropdown next to Check.
- Changed the source policy so Markdown documentation may name the product and the organization.
- Changed `bun run dev` so it never writes outside the repository. It previously installed a desktop entry, an icon and AppStream metadata into the user data directory and refreshed the desktop caches.
- Changed development mode to default off outside `bun run dev`, so production launches print to the selected queue unless the user enabled development output.
- Changed ticket cards and print-page ticket actions to use even padding and standard bordered buttons, and renamed the home action to Print.
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
