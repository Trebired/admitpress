use super::*;
use chrono::{Datelike, NaiveDateTime};

const KEEP_ALL_HOURS: i64 = 24;
const KEEP_DAILY_DAYS: i64 = 7;
const KEEP_WEEKLY_DAYS: i64 = 30;

pub(in crate::service) fn backup_stamp(file_name: &str) -> Option<NaiveDateTime> {
    let stamp = file_name
    .strip_prefix(BACKUP_FILE_PREFIX)?
    .strip_suffix(BACKUP_FILE_SUFFIX)?;
    NaiveDateTime::parse_from_str(stamp, BACKUP_STAMP_FORMAT).ok()
}

pub(super) fn stored_backups(directory: &Path) -> Result<Vec<(PathBuf, NaiveDateTime)>> {
    let Ok(entries) = fs::read_dir(directory) else {
        return Ok(Vec::new());
    };
    let mut stored = Vec::new();
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(stamp) = backup_stamp(&name) {
            stored.push((entry.path(), stamp));
        }
    }
    stored.sort_by(|left, right| left.1.cmp(&right.1));
    Ok(stored)
}

pub(in crate::service) fn expired_backups(
    stored: &[(PathBuf, NaiveDateTime)],
    now: NaiveDateTime,
) -> Vec<PathBuf> {
    let mut kept = HashSet::new();
    let mut expired = Vec::new();
    for (path, stamp) in stored.iter().rev() {
        let keep = retention_bucket(*stamp, now).is_some_and(|bucket| kept.insert(bucket));
        if !keep {
            expired.push(path.clone());
        }
    }
    expired
}

fn retention_bucket(stamp: NaiveDateTime, now: NaiveDateTime) -> Option<String> {
    let age = now.signed_duration_since(stamp);
    if age.num_hours() < KEEP_ALL_HOURS {
        return Some(format!("exact-{}", stamp.format(BACKUP_STAMP_FORMAT)));
    }
    if age.num_days() < KEEP_DAILY_DAYS {
        return Some(format!("day-{}", stamp.date()));
    }
    if age.num_days() <= KEEP_WEEKLY_DAYS {
        let week = stamp.iso_week();
        return Some(format!("week-{}-{}", week.year(), week.week()));
    }
    None
}

pub(super) fn backup_info(path: &Path) -> Result<BackupInfo> {
    let file_name = path
    .file_name()
    .map(|name| name.to_string_lossy().into_owned())
    .unwrap_or_default();
    let created_at = backup_stamp(&file_name)
    .and_then(|stamp| stamp.and_local_timezone(Local).single())
    .map(|stamp| stamp.to_rfc3339())
    .unwrap_or_default();
    Ok(BackupInfo {
            file_name,
            path: path.to_string_lossy().into_owned(),
            created_at,
            size_bytes: fs::metadata(path).map(|data| data.len()).unwrap_or(0),
    })
}
