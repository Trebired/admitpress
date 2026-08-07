use super::*;

pub(super) fn configured_data_dir_file() -> Option<PathBuf> {
    let config_directory = config_directory_name();
    if let Some(path) = std::env::var_os("XDG_CONFIG_HOME") {
        return Some(PathBuf::from(path).join(&config_directory).join("data-dir"));
    }
    std::env::var_os("HOME")
    .map(PathBuf::from)
    .map(|path| path.join(".config").join(config_directory).join("data-dir"))
}

fn config_directory_name() -> String {
    std::env::var("TICKET_APP_CONFIG_DIR_NAME")
    .ok()
    .filter(|value| !value.trim().is_empty())
    .unwrap_or_else(|| "native-ticket".to_owned())
}

pub(super) fn default_data_dir() -> PathBuf {
    let directory = config_directory_name();
    if let Some(path) = std::env::var_os("XDG_DATA_HOME") {
        return PathBuf::from(path).join(directory);
    }
    std::env::var_os("HOME")
    .map(PathBuf::from)
    .map(|path| path.join(".local").join("share").join(directory))
    .unwrap_or_else(|| PathBuf::from("data"))
}

pub(super) fn read_configured_data_dir() -> Option<PathBuf> {
    let config_file = configured_data_dir_file()?;
    let value = fs::read_to_string(config_file).ok()?;
    let value = value.trim();
    if value.is_empty() {
        None
    } else {
        Some(PathBuf::from(value))
    }
}

pub(super) fn write_configured_data_dir(data_dir: &Path) -> Result<()> {
    let config_file = configured_data_dir_file()
    .ok_or_else(|| anyhow!("HOME or XDG_CONFIG_HOME is required to save the data directory"))?;
    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(config_file, data_dir.to_string_lossy().as_bytes())?;
    Ok(())
}
