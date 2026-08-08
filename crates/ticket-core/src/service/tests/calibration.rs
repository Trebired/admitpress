use super::*;

fn save_with_queue(
    service: &mut TicketService,
    summary: &TicketSummary,
    width_cm: f64,
    height_cm: f64,
    queue: Option<&str>,
) {
    service
    .save_ticket(
        &summary.ticket.id,
        &summary.ticket.name,
        width_cm,
        height_cm,
        queue,
        &summary.ticket.border_style,
        summary.ticket.border_width_mm,
        summary.ticket.dpi,
        summary.ticket.number_start,
        summary.ticket.number_increment,
        summary.ticket.number_padding,
        &summary.ticket.number_prefix,
        &summary.ticket.number_suffix,
        &field_inputs(summary),
        &subticket_inputs(summary),
    )
    .expect("ticket should save");
}

#[test]
fn a_ticket_without_a_printer_keeps_its_own_page_size() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    service
    .set_printer_page_size("PF8t", 10.0, 4.0)
    .expect("printer size should save");

    save_with_queue(&mut service, &summary, 8.0, 5.0, None);

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should reload");
    assert_eq!(after.ticket.width_mm, 80.0);
    assert_eq!(after.ticket.height_mm, 50.0);
    assert_eq!(after.ticket.printer_queue, None);
}

#[test]
fn a_ticket_assigned_to_a_printer_uses_that_printers_page_size() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    service
    .set_printer_page_size("PF8t", 10.0, 4.0)
    .expect("printer size should save");

    save_with_queue(&mut service, &summary, 8.0, 5.0, Some("PF8t"));
    service
    .set_printer_page_size("PF8t", 6.0, 3.0)
    .expect("printer size should update");

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should reload");
    assert_eq!(after.ticket.width_mm, 60.0);
    assert_eq!(after.ticket.height_mm, 30.0);
}

#[test]
fn saving_an_assigned_ticket_resizes_its_printer() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");

    save_with_queue(&mut service, &summary, 9.0, 6.0, Some("PF8t"));

    let profile = service
    .printer_profile("PF8t")
    .expect("profile should load")
    .expect("profile should exist");
    assert_eq!(profile.width_mm, 90.0);
    assert_eq!(profile.height_mm, 60.0);
}

#[test]
fn an_unknown_printer_falls_back_to_the_default_page_size() {
    let service = service();
    let page = service
    .printer_page_size("never-configured")
    .expect("page size should resolve");
    assert_eq!(page.width_mm, 50.0);
    assert_eq!(page.height_mm, 70.0);
}

#[test]
fn calibration_values_are_clamped_to_a_safe_range() {
    let mut service = service();
    let profile = service
    .set_printer_calibration(
        "PF8t",
        PrintCalibration {
            offset_x_mm: 900.0,
            offset_y_mm: -900.0,
            scale_x_percent: 500.0,
            scale_y_percent: 1.0,
        },
    )
    .expect("calibration should save");

    assert_eq!(profile.offset_x_mm, 50.0);
    assert_eq!(profile.offset_y_mm, -50.0);
    assert_eq!(profile.scale_x_percent, 120.0);
    assert_eq!(profile.scale_y_percent, 80.0);
}

#[test]
fn calibration_shifts_printed_output_but_not_the_preview() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    save_with_queue(&mut service, &summary, 8.0, 5.0, Some("PF8t"));
    service
    .set_printer_calibration(
        "PF8t",
        PrintCalibration {
            offset_x_mm: 1.5,
            offset_y_mm: -0.5,
            scale_x_percent: 101.0,
            scale_y_percent: 100.0,
        },
    )
    .expect("calibration should save");

    let printed = service
    .print_one(&ticket.id, &summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("ticket should print");
    let output = printed
    .record
    .rendered_output_path
    .expect("development output should exist");
    let svg = fs::read_to_string(&output).expect("printed svg should read");
    assert!(svg.contains("translate(1.5 -0.5)"), "{svg}");

    let preview_url = service
    .preview_ticket(&ticket.id, &summary.subtickets[0].id, "2026-08-02", 1)
    .expect("preview should render");
    let preview_path = PathBuf::from(decode_file_path(&preview_url)).with_extension("svg");
    let preview_svg = fs::read_to_string(&preview_path).expect("preview svg should read");
    assert!(!preview_svg.contains("translate("), "{preview_svg}");
}

#[test]
fn the_calibration_sheet_carries_rulers_and_the_queue_name() {
    let mut service = service();

    let output = service
    .print_calibration_sheet(
        "PF8t",
        PageSize {
            width_mm: 80.0,
            height_mm: 50.0,
        },
        PrintCalibration::none(),
    )
    .expect("calibration sheet should render");
    let svg = fs::read_to_string(&output).expect("calibration svg should read");

    assert!(svg.contains("80mm"), "{svg}");
    assert!(svg.contains("50mm"), "{svg}");
    assert!(svg.contains("PF8t"), "{svg}");
    assert!(svg.contains("<line"), "{svg}");
}

#[test]
fn the_calibration_sheet_uses_the_values_passed_in_not_the_saved_profile() {
    let mut service = service();
    service
    .set_printer_page_size("PF8t", 8.0, 5.0)
    .expect("printer size should save");
    service
    .set_printer_calibration(
        "PF8t",
        PrintCalibration {
            offset_x_mm: 3.0,
            offset_y_mm: 3.0,
            scale_x_percent: 110.0,
            scale_y_percent: 110.0,
        },
    )
    .expect("calibration should save");

    let output = service
    .print_calibration_sheet(
        "PF8t",
        PageSize {
            width_mm: 60.0,
            height_mm: 40.0,
        },
        PrintCalibration {
            offset_x_mm: 1.0,
            offset_y_mm: -1.0,
            scale_x_percent: 100.0,
            scale_y_percent: 100.0,
        },
    )
    .expect("calibration sheet should render");
    let svg = fs::read_to_string(&output).expect("calibration svg should read");

    assert!(svg.contains("60mm"), "{svg}");
    assert!(svg.contains("40mm"), "{svg}");
    assert!(svg.contains("translate(1 -1)"), "{svg}");
    assert!(!svg.contains("80mm"), "{svg}");
    assert!(!svg.contains("translate(3 3)"), "{svg}");
}
