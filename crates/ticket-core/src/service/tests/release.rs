use super::*;
use crate::service::release::version::{is_newer, parse_version};

#[test]
fn version_parsing_accepts_tags_and_rejects_junk() {
    assert_eq!(parse_version("v1.2.3"), Some(vec![1, 2, 3]));
    assert_eq!(parse_version("0.1.0"), Some(vec![0, 1, 0]));
    assert_eq!(parse_version("2.0.0-beta.1"), Some(vec![2, 0, 0]));
    assert_eq!(parse_version("nightly"), None);
    assert_eq!(parse_version(""), None);
}

#[test]
fn newer_versions_compare_by_number_not_by_text() {
    assert!(is_newer("0.10.0", "0.9.0"));
    assert!(is_newer("1.0.0", "0.99.99"));
    assert!(is_newer("0.1.1", "0.1.0"));
    assert!(!is_newer("0.1.0", "0.1.0"));
    assert!(!is_newer("0.1.0", "0.2.0"));
    assert!(!is_newer("garbage", "0.1.0"));
}

#[test]
fn shorter_versions_are_padded_with_zeros() {
    assert!(is_newer("1.1", "1.0.9"));
    assert!(!is_newer("1.0", "1.0.0"));
    assert!(is_newer("2", "1.9.9"));
}

#[test]
fn release_status_reports_no_update_before_any_check() {
    let service = service();
    let status = service.release_status("0.1.0").expect("status should read");
    assert_eq!(status.current_version, "0.1.0");
    assert_eq!(status.latest_version, None);
    assert!(!status.update_available);
    assert_eq!(status.checked_at, None);
}

#[test]
fn a_stored_newer_release_is_reported_as_an_update() {
    let service = service();
    service
    .storage
    .set_setting("latest_release", "0.4.0")
    .expect("setting should store");
    let status = service.release_status("0.1.0").expect("status should read");
    assert_eq!(status.latest_version.as_deref(), Some("0.4.0"));
    assert!(status.update_available);
}

#[test]
fn an_unreachable_repository_records_the_attempt_without_failing() {
    let service = service();
    let status = service
    .refresh_release_status("0.1.0", "https://example.invalid/not/a/repo")
    .expect("refresh should not fail when the repository is unreachable");
    assert_eq!(status.latest_version, None);
    assert!(!status.update_available);
    assert!(status.checked_at.is_some());
}
