use super::*;

pub(super) fn archive_destination_path(destination_path: &str) -> PathBuf {
    let path = PathBuf::from(decode_file_path(destination_path));
    let text = path.to_string_lossy();
    if text.to_lowercase().ends_with(".tar.gz") {
        path
    } else {
        PathBuf::from(format!("{text}.tar.gz"))
    }
}

pub(super) fn imported_print_output_path(
    data_dir: &Path,
    ticket_id: &str,
    record_id: &str,
    local_date: &str,
    original_path: &str,
) -> PathBuf {
    let extension = path_extension(original_path).unwrap_or_default();
    let root = if extension == "svg" {
        "development-prints"
    } else {
        "print-jobs"
    };
    data_dir
    .join(root)
    .join(local_date)
    .join(ticket_id)
    .join(archive_file_name(original_path, record_id))
}

pub(super) fn archive_file_name(path: &str, fallback_stem: &str) -> String {
    let file_name = Path::new(path)
    .file_name()
    .and_then(|value| value.to_str())
    .map(safe_archive_component)
    .filter(|value| !value.is_empty());
    match file_name {
        Some(file_name) => file_name,
        None => match path_extension(path) {
            Some(extension) => format!("{}.{}", safe_archive_component(fallback_stem), extension),
            None => safe_archive_component(fallback_stem),
        },
    }
}

pub(super) fn safe_archive_component(value: &str) -> String {
    let cleaned = value
    .chars()
    .map(|character| match character {
            '/' | '\\' | '\0' => '_',
            other => other,
    })
    .collect::<String>();
    if cleaned.trim().is_empty() {
        "file".to_owned()
    } else {
        cleaned
    }
}

pub(super) fn path_extension(path: &str) -> Option<String> {
    Path::new(path)
    .extension()
    .and_then(|value| value.to_str())
    .map(|value| value.trim().to_lowercase())
    .filter(|value| !value.is_empty())
}
