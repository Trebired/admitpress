use super::*;

pub(super) fn collect_ticket_archive_assets(
    ticket: &Ticket,
    fields: &[TicketField],
    print_records: &[PrintRecord],
) -> Vec<TicketArchiveAsset> {
    let mut assets = Vec::new();
    let mut seen_paths = HashMap::new();
    if let Some(path) = ticket.background_svg_path.as_deref() {
        let extension = path_extension(path).unwrap_or_else(|| "svg".to_owned());
        push_archive_asset(
            &mut assets,
            &mut seen_paths,
            "background",
            path,
            format!("assets/background/background.{extension}"),
        );
    }
    for field in fields {
        if field.field_kind != FieldKind::Image || field.text.trim().is_empty() {
            continue;
        }
        let extension = match path_extension(&field.text) {
            Some(extension) => extension,
            None => continue,
        };
        push_archive_asset(
            &mut assets,
            &mut seen_paths,
            "image",
            &field.text,
            format!(
                "assets/images/{}.{}",
                safe_archive_component(&field.id),
                extension
            ),
        );
    }
    for record in print_records {
        let Some(path) = record.rendered_output_path.as_deref() else {
            continue;
        };
        push_archive_asset(
            &mut assets,
            &mut seen_paths,
            "output",
            path,
            format!(
                "assets/outputs/{}/{}",
                safe_archive_component(&record.id),
                archive_file_name(path, &record.id)
            ),
        );
    }
    assets
}

pub(super) fn push_archive_asset(
    assets: &mut Vec<TicketArchiveAsset>,
    seen_paths: &mut HashMap<String, String>,
    kind: &str,
    path: &str,
    archive_path: String,
) {
    let trimmed = path.trim();
    if trimmed.is_empty() || !Path::new(trimmed).is_file() || seen_paths.contains_key(trimmed) {
        return;
    }
    seen_paths.insert(trimmed.to_owned(), archive_path.clone());
    assets.push(TicketArchiveAsset {
            kind: kind.to_owned(),
            original_path: trimmed.to_owned(),
            archive_path,
    });
}
