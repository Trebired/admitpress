use super::*;

#[test]
fn selected_printer_queue_is_app_setting() {
    let path = std::env::temp_dir().join(format!("ticket-service-test-{}", Uuid::new_v4()));
    let mut service = TicketService::open_at(path.clone()).expect("service should open");

    service
    .set_selected_printer_queue(Some("PF8T"))
    .expect("printer queue should save");
    assert_eq!(service.selected_printer_queue(), Some("PF8T"));

    let mut reopened = TicketService::open_at(path.clone()).expect("service should reopen");
    assert_eq!(reopened.selected_printer_queue(), Some("PF8T"));

    reopened
    .set_selected_printer_queue(None)
    .expect("printer queue should clear");
    let cleared = TicketService::open_at(path).expect("service should reopen");
    assert_eq!(cleared.selected_printer_queue(), None);
}

#[test]
fn cups_media_size_uses_ticket_dimensions() {
    let mut service = service();
    let ticket = service
    .create_ticket("Small label", 7.0, 5.0)
    .expect("ticket should be created");

    assert_eq!(cups_media_size(&ticket), "Custom.70x50mm");
}

#[test]
fn cups_job_options_center_exact_size_pdf_on_media() {
    let mut service = service();
    let ticket = service
    .create_ticket("Small label", 7.0, 5.0)
    .expect("ticket should be created");

    let options = cups_job_options(&ticket);

    assert!(options.contains(&"media=Custom.70x50mm".to_owned()));
    assert!(options.contains(&"PageSize=Custom.70x50mm".to_owned()));
    assert!(options.contains(&"position=center".to_owned()));
    assert!(!options.contains(&"position=top-left".to_owned()));
}

#[test]
fn reopening_repairs_legacy_centered_field_text_alignment() {
    let path = std::env::temp_dir().join(format!("ticket-service-test-{}", Uuid::new_v4()));
    let mut service = TicketService::open_at(path.clone()).expect("service should open");
    let ticket = service
    .create_ticket("Legacy label", 5.0, 9.0)
    .expect("ticket should be created");
    let recorded_at = now_iso();
    service
    .storage
    .replace_fields(
        &ticket.id,
        &[legacy_centered_field(), legacy_full_width_field()],
        &recorded_at,
    )
    .expect("fields should save");

    let mut reopened = TicketService::open_at(path).expect("service should reopen");
    let summary = reopened
    .ticket_summary(&ticket.id, &today_local())
    .expect("summary should load");

    assert_eq!(summary.fields[0].horizontal_alignment, "center");
    assert_eq!(summary.fields[1].horizontal_alignment, "left");
}

fn legacy_centered_field() -> TicketFieldInput {
    legacy_field("centered URL", 6.25, 60.0, 37.5, 0)
}

fn legacy_full_width_field() -> TicketFieldInput {
    legacy_field("full width", 0.0, 5.0, 50.0, 1)
}

fn legacy_field(
    text: &str,
    x_mm: f64,
    y_mm: f64,
    width_mm: f64,
    sort_order: i64,
) -> TicketFieldInput {
    TicketFieldInput {
        id: String::new(),
        template_kind: "classic".into(),
        field_kind: FieldKind::Static,
        dynamic_kind: None,
        text: text.into(),
        x_mm,
        y_mm,
        width_mm,
        height_mm: 8.0,
        font_family: "Sans Serif".into(),
        font_size_pt: 12.0,
        font_weight: 400,
        horizontal_alignment: "left".into(),
        vertical_alignment: "center".into(),
        date_format: "date".into(),
        prefix: String::new(),
        suffix: String::new(),
        sort_order,
    }
}

#[test]
fn print_pdf_uses_ticket_page_size() {
    if Command::new("pdfinfo").arg("-v").output().is_err()
    || (Command::new("rsvg-convert")
        .arg("--version")
        .output()
        .is_err()
        && Command::new("inkscape").arg("--version").output().is_err())
    {
        return;
    }

    let mut service = service();
    let ticket = service
    .create_ticket("Small label", 7.0, 5.0)
    .expect("ticket should be created");
    let svg_path = service.data_dir().join("page-size-test.svg");
    let pdf_path = service.data_dir().join("page-size-test.pdf");
    fs::write(
        &svg_path,
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="70mm" height="50mm" viewBox="0 0 70 50"><rect width="70" height="50" "#,
            r#"fill="white"/></svg>"#
        ),
    )
    .expect("test SVG should write");

    render_print_pdf(&svg_path, &pdf_path, &ticket).expect("print PDF should render");
    let (width, height) = pdf_page_size_points(&pdf_path).expect("PDF page size should parse");
    assert!(
        (width - 70.0 / 25.4 * 72.0).abs() < 0.5,
        "width was {width}"
    );
    assert!(
        (height - 50.0 / 25.4 * 72.0).abs() < 0.5,
        "height was {height}"
    );
}

fn pdf_page_size_points(path: &Path) -> Option<(f64, f64)> {
    let output = Command::new("pdfinfo").arg(path).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout
    .lines()
    .find(|line| line.trim_start().starts_with("Page size:"))?;
    let mut values = line
    .split_whitespace()
    .filter_map(|part| part.parse::<f64>().ok());
    Some((values.next()?, values.next()?))
}
