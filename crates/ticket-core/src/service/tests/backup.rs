use super::*;
use crate::service::backup::retention::{backup_stamp, expired_backups};
use chrono::NaiveDateTime;
use std::io::Write;

fn stamp_at(value: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S").expect("stamp should parse")
}

fn synthetic(names: &[&str]) -> Vec<(PathBuf, NaiveDateTime)> {
    let mut stored: Vec<(PathBuf, NaiveDateTime)> = names
    .iter()
    .map(|name| (PathBuf::from(*name), backup_stamp(name).expect("name should parse")))
    .collect();
    stored.sort_by(|left, right| left.1.cmp(&right.1));
    stored
}

fn extract_member(archive_path: &Path, member: &str) -> Vec<u8> {
    let file = fs::File::open(archive_path).expect("backup should open");
    let mut archive = Archive::new(GzDecoder::new(file));
    for entry in archive.entries().expect("entries should read") {
        let mut entry = entry.expect("entry should read");
        let path = entry.path().expect("path should read").into_owned();
        if path == Path::new(member) {
            let mut bytes = Vec::new();
            entry.read_to_end(&mut bytes).expect("member should read");
            return bytes;
        }
    }
    panic!("{member} is missing from the backup");
}

#[test]
fn backup_captures_writes_still_held_in_the_wal() {
    let mut service = service();
    let ticket = service
    .create_ticket("Wal Proof", 8.0, 5.0)
    .expect("ticket should be created");

    let info = service.create_backup().expect("backup should be created");

    let database_bytes = extract_member(Path::new(&info.path), "application.sqlite");
    let restored = std::env::temp_dir().join(format!("ticket-backup-test-{}.sqlite", Uuid::new_v4()));
    fs::File::create(&restored)
    .expect("restore target should be created")
    .write_all(&database_bytes)
    .expect("restore target should be written");

    let storage = Storage::open(&restored).expect("restored database should open");
    let tickets = storage.list_tickets().expect("tickets should list");
    assert!(tickets.iter().any(|entry| entry.id == ticket.id));
}

#[test]
fn backup_writes_a_manifest_and_records_the_time() {
    let service = service();
    let info = service.create_backup().expect("backup should be created");

    let manifest_bytes = extract_member(Path::new(&info.path), "backup.json");
    let manifest: BackupManifest =
    serde_json::from_slice(&manifest_bytes).expect("manifest should parse");
    assert_eq!(manifest.format, "data.backup");
    assert!(info.size_bytes > 0);
    assert!(service.last_backup_at().expect("setting should read").is_some());
}

#[test]
fn backup_is_due_before_the_first_run_and_not_right_after_one() {
    let service = service();
    assert!(service.backup_due().expect("due should resolve"));
    service.create_backup().expect("backup should be created");
    assert!(!service.backup_due().expect("due should resolve"));
}

#[test]
fn pruning_thins_older_backups_and_keeps_recent_ones() {
    let now = stamp_at("2026-03-02 12:00:00");
    let stored = synthetic(&[
            "backup-20260302-110000.tar.gz",
            "backup-20260302-080000.tar.gz",
            "backup-20260228-090000.tar.gz",
            "backup-20260228-210000.tar.gz",
            "backup-20260210-030000.tar.gz",
            "backup-20260210-150000.tar.gz",
            "backup-20251220-030000.tar.gz",
    ]);

    let expired: Vec<String> = expired_backups(&stored, now)
    .iter()
    .map(|path| path.to_string_lossy().into_owned())
    .collect();

    assert_eq!(
        expired,
        vec![
            "backup-20260228-090000.tar.gz",
            "backup-20260210-030000.tar.gz",
            "backup-20251220-030000.tar.gz",
        ]
    );
}

#[test]
fn listing_backups_reports_the_newest_first() {
    let service = service();
    let directory = service.backups_dir();
    for name in ["backup-20260301-090000.tar.gz", "backup-20260302-090000.tar.gz"] {
        fs::write(directory.join(name), b"placeholder").expect("placeholder should write");
    }

    let backups = service.list_backups().expect("backups should list");
    assert_eq!(backups[0].file_name, "backup-20260302-090000.tar.gz");
    assert_eq!(backups.len(), 2);
}
