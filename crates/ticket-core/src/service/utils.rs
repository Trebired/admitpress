use super::*;

pub(super) fn append_archive_bytes(
    builder: &mut Builder<GzEncoder<fs::File>>,
    path: &str,
    bytes: &[u8],
) -> Result<()> {
    let mut header = Header::new_gnu();
    header.set_mode(0o644);
    header.set_size(bytes.len() as u64);
    header.set_cksum();
    builder
    .append_data(&mut header, path, Cursor::new(bytes))
    .with_context(|| format!("failed to add {path} to the archive"))
}

pub(super) fn normalized_optional(value: impl AsRef<str>) -> Option<String> {
    let value = value.as_ref().trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_owned())
    }
}

pub(super) fn normalize_border_style(value: &str) -> &str {
    match value.trim().to_lowercase().as_str() {
        "solid" => "solid",
        "dashed" => "dashed",
        "dotted" => "dotted",
        _ => "none",
    }
}

pub(super) fn normalize_border_width_mm(value: f64) -> f64 {
    if value.is_finite() {
        value.clamp(0.1, 5.0)
    } else {
        0.2
    }
}

pub(super) fn validate_date(value: &str) -> Result<()> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
    .map(|_| ())
    .map_err(|_| TicketError::InvalidDate.into())
}

pub(super) fn now_iso() -> String {
    Local::now().to_rfc3339()
}

pub(super) fn today_local() -> String {
    Local::now().date_naive().format("%Y-%m-%d").to_string()
}

pub(super) fn decode_file_path(value: &str) -> String {
    let raw = value.strip_prefix("file://").unwrap_or(value);
    let bytes = raw.as_bytes();
    let mut output = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            let high = hex_value(bytes[index + 1]);
            let low = hex_value(bytes[index + 2]);
            if let (Some(high), Some(low)) = (high, low) {
                output.push((high << 4) | low);
                index += 3;
                continue;
            }
        }
        output.push(bytes[index]);
        index += 1;
    }
    String::from_utf8_lossy(&output).into_owned()
}

pub(super) fn hex_value(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}

pub(super) fn path_to_file_url(path: &Path) -> String {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let text = path.to_string_lossy();
    let mut encoded = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
                encoded.push(byte as char);
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    format!("file://{encoded}")
}

pub(super) fn preview_path_component(value: &str) -> String {
    let cleaned = value
    .chars()
    .map(|character| match character {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => character,
            _ => '-',
    })
    .collect::<String>();
    let trimmed = cleaned.trim_matches('-');
    if trimmed.is_empty() {
        "preview".to_owned()
    } else {
        trimmed.to_owned()
    }
}
