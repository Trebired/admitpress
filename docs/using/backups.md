# Backups

Backups are their own item in the sidebar.

## Schedule

A backup is written every three hours while the application runs, and once at
startup if the newest copy is older than three hours. Nothing needs to be
configured.

## What a backup contains

`backups/backup-YYYYMMDD-HHMMSS.tar.gz` in the data directory, holding:

| Entry | What |
| --- | --- |
| `application.sqlite` | A consistent snapshot of the database |
| `tickets/` | Ticket backgrounds and imported images |
| `backup.json` | Format, version and creation time |

The database is captured with SQLite's `VACUUM INTO`, not copied. A plain copy
would miss everything still sitting in the write-ahead log, which is routinely
larger than the database file itself.

Rendered print jobs, previews and development output are excluded. They
regenerate, and including them would multiply the archive size for no gain.

## Thinning

Old copies are removed automatically:

| Age | Kept |
| --- | --- |
| Under a day | Every copy |
| One day to one week | The newest of each day |
| One week to one month | The newest of each week |
| Over a month | None |

This settles at roughly twenty archives regardless of how long the application
has been in use.

## The dialog

Sidebar, **Backups**:

- **Last backup**, in your locale's date and time format
- **Stored copies**, with the total size on disk
- **Folder**, the full path
- **Back up now**, which writes one immediately

A failed automatic backup is shown here as an error, rather than passing by as
a notification you might miss.

## Restoring

Restoring is deliberately not in the application, because it must replace a
live database rather than merge into one.

To restore:

1. Close AdmitPress
2. Extract the archive over the data directory, replacing `application.sqlite`
   and `tickets/`
3. Start AdmitPress

```sh
cd ~/.local/share/admitpress
tar xzf backups/backup-20260807-120000.tar.gz
```

Delete `application.sqlite-wal` and `application.sqlite-shm` first if they
exist, so the restored database is not reopened against a stale log.
