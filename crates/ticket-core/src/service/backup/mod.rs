use super::*;

pub(super) mod retention;

use retention::*;

const BACKUP_FORMAT: &str = "data.backup";
const BACKUP_VERSION: u32 = 1;
const BACKUP_MANIFEST: &str = "backup.json";
const BACKUP_DATABASE: &str = "application.sqlite";
const BACKUP_ASSETS: &str = "tickets";
const BACKUP_INTERVAL_SECONDS: i64 = 3 * 60 * 60;
const BACKUP_FILE_PREFIX: &str = "backup-";
const BACKUP_FILE_SUFFIX: &str = ".tar.gz";
const BACKUP_STAMP_FORMAT: &str = "%Y%m%d-%H%M%S";
const LAST_BACKUP_SETTING: &str = "last_backup_at";

impl TicketService {
    pub fn backups_dir(&self) -> PathBuf {
        self.data_dir.join("backups")
    }

    pub fn create_backup(&self) -> Result<BackupInfo> {
        let created = Local::now();
        let directory = self.backups_dir();
        fs::create_dir_all(&directory)?;
        let stamp = created.format(BACKUP_STAMP_FORMAT);
        let destination =
        directory.join(format!("{BACKUP_FILE_PREFIX}{stamp}{BACKUP_FILE_SUFFIX}"));
        let snapshot = directory.join("snapshot.sqlite.part");
        self.storage.snapshot_database(&snapshot)?;
        let written = self.write_backup_archive(&destination, &snapshot, &created.to_rfc3339());
        let _ = fs::remove_file(&snapshot);
        if written.is_err() {
            let _ = fs::remove_file(&destination);
        }
        written?;
        self.storage
        .set_setting(LAST_BACKUP_SETTING, &created.to_rfc3339())?;
        self.prune_backups()?;
        backup_info(&destination)
    }

    fn write_backup_archive(
        &self,
        destination: &Path,
        snapshot: &Path,
        created_at: &str,
    ) -> Result<()> {
        let file = fs::File::create(destination)
        .with_context(|| format!("failed to create backup at {}", destination.display()))?;
        let mut builder = Builder::new(GzEncoder::new(file, Compression::default()));
        let manifest = BackupManifest {
            format: BACKUP_FORMAT.to_owned(),
            version: BACKUP_VERSION,
            created_at: created_at.to_owned(),
        };
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
        append_archive_bytes(&mut builder, BACKUP_MANIFEST, &manifest_bytes)?;
        builder
        .append_path_with_name(snapshot, BACKUP_DATABASE)
        .context("failed to add the database snapshot to the backup")?;
        let assets = self.data_dir.join(BACKUP_ASSETS);
        if assets.is_dir() {
            builder
            .append_dir_all(BACKUP_ASSETS, &assets)
            .context("failed to add the ticket assets to the backup")?;
        }
        builder.into_inner()?.finish()?;
        Ok(())
    }

    pub fn list_backups(&self) -> Result<Vec<BackupInfo>> {
        let mut backups = Vec::new();
        for (path, _) in stored_backups(&self.backups_dir())? {
            backups.push(backup_info(&path)?);
        }
        backups.reverse();
        Ok(backups)
    }

    pub fn prune_backups(&self) -> Result<usize> {
        let stored = stored_backups(&self.backups_dir())?;
        let expired = expired_backups(&stored, Local::now().naive_local());
        let mut removed = 0;
        for path in expired {
            if fs::remove_file(&path).is_ok() {
                removed += 1;
            }
        }
        Ok(removed)
    }

    pub fn backup_due(&self) -> Result<bool> {
        let Some(last) = self.storage.get_setting(LAST_BACKUP_SETTING)? else {
            return Ok(true);
        };
        let Ok(parsed) = DateTime::parse_from_rfc3339(&last) else {
            return Ok(true);
        };
        let elapsed = Local::now().signed_duration_since(parsed).num_seconds();
        Ok(elapsed >= BACKUP_INTERVAL_SECONDS || elapsed < 0)
    }

    pub fn last_backup_at(&self) -> Result<Option<String>> {
        self.storage.get_setting(LAST_BACKUP_SETTING)
    }
}
