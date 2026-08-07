pub(crate) fn normalize_template_kind(value: &str) -> &str {
    match value.trim() {
        "bulk" => "bulk",
        _ => "classic",
    }
}

pub(crate) fn valid_template_kind(value: &str) -> bool {
    matches!(value.trim(), "" | "classic" | "bulk")
}
