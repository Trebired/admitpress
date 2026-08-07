use super::*;
use serde_json::Value;

pub(in crate::service) mod version;

use version::is_newer;

const RELEASE_CHECK_TTL_SECONDS: i64 = 3 * 60 * 60;
const RELEASE_CHECK_TIMEOUT: Duration = Duration::from_secs(6);
const LATEST_RELEASE_SETTING: &str = "latest_release";
const LATEST_RELEASE_URL_SETTING: &str = "latest_release_url";
const RELEASE_CHECKED_SETTING: &str = "latest_release_checked_at";

impl TicketService {
    pub fn release_status(&self, current_version: &str) -> Result<ReleaseStatus> {
        let latest = self.storage.get_setting(LATEST_RELEASE_SETTING)?;
        let checked_at = self.storage.get_setting(RELEASE_CHECKED_SETTING)?;
        let release_url = self.storage.get_setting(LATEST_RELEASE_URL_SETTING)?;
        Ok(build_release_status(current_version, latest, checked_at, release_url))
    }

    pub fn refresh_release_status(&self, current_version: &str, repository: &str) -> Result<ReleaseStatus> {
        if !self.release_check_due()? {
            return self.release_status(current_version);
        }
        if let Some(latest) = fetch_latest_release(repository) {
            self.storage
            .set_setting(LATEST_RELEASE_SETTING, &latest.version)?;
            self.storage
            .set_setting(LATEST_RELEASE_URL_SETTING, &latest.url)?;
        }
        self.storage
        .set_setting(RELEASE_CHECKED_SETTING, &now_iso())?;
        self.release_status(current_version)
    }

    fn release_check_due(&self) -> Result<bool> {
        let Some(checked_at) = self.storage.get_setting(RELEASE_CHECKED_SETTING)? else {
            return Ok(true);
        };
        let Ok(parsed) = DateTime::parse_from_rfc3339(&checked_at) else {
            return Ok(true);
        };
        let elapsed = Local::now().signed_duration_since(parsed).num_seconds();
        Ok(elapsed >= RELEASE_CHECK_TTL_SECONDS || elapsed < 0)
    }
}

fn build_release_status(
    current_version: &str,
    latest: Option<String>,
    checked_at: Option<String>,
    release_url: Option<String>,
) -> ReleaseStatus {
    let latest = latest.filter(|value| !value.is_empty());
    let update_available = latest
    .as_deref()
    .is_some_and(|value| is_newer(value, current_version));
    ReleaseStatus {
        current_version: current_version.to_owned(),
        latest_version: latest,
        update_available,
        checked_at,
        release_url: release_url.filter(|value| !value.is_empty()),
    }
}

struct LatestRelease {
    version: String,
    url: String,
}

fn fetch_latest_release(repository: &str) -> Option<LatestRelease> {
    let slug = repository_slug(repository)?;
    let agent = ureq::Agent::config_builder()
    .timeout_global(Some(RELEASE_CHECK_TIMEOUT))
    .build()
    .new_agent();
    let mut response = agent
    .get(&format!("https://api.github.com/repos/{slug}/releases/latest"))
    .header("Accept", "application/vnd.github+json")
    .header("User-Agent", &slug)
    .call()
    .ok()?;
    let body: Value = response.body_mut().read_json().ok()?;
    let tag = body.get("tag_name").and_then(Value::as_str)?;
    version::parse_version(tag)?;
    let url = body
    .get("html_url")
    .and_then(Value::as_str)
    .map(str::to_owned)
    .unwrap_or_else(|| format!("https://github.com/{slug}/releases/latest"));
    Some(LatestRelease {
            version: tag.trim_start_matches(['v', 'V']).to_owned(),
            url,
    })
}

fn repository_slug(repository: &str) -> Option<String> {
    let trimmed = repository
    .trim()
    .trim_end_matches('/')
    .trim_end_matches(".git");
    let (_, slug) = trimmed.split_once("github.com")?;
    let slug = slug.trim_start_matches(['/', ':']);
    (slug.split('/').filter(|part| !part.is_empty()).count() == 2).then(|| slug.to_owned())
}
