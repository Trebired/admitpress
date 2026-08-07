use super::*;

#[test]
fn exports_and_imports_complete_ticket_archive() {
    let mut service = service();
    let local_date = today_local();
    let ticket = create_archive_ticket(&mut service);
    let summary = sample_ticket_summary(&mut service, &ticket.id, &local_date);
    print_archive_sample(
        &mut service,
        &ticket.id,
        &summary.subtickets[0].id,
        &local_date,
    );

    let imported = roundtrip_archive(&mut service, &ticket.id);

    assert_imported_ticket(&imported);
    assert_ne!(imported.id, ticket.id);
    let imported_summary = service
    .ticket_summary(&imported.id, &local_date)
    .expect("imported summary should load");
    assert_imported_summary(&imported_summary, &summary, &imported.id);
}

fn create_archive_ticket(service: &mut TicketService) -> Ticket {
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let source_background = service.data_dir().join("source-background.svg");
    fs::write(
        &source_background,
        concat!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="80mm" height="50mm" viewBox="0 0 80 50"><rect width="80" height="50" "#,
            r#"fill="white"/><circle cx="20" cy="20" r="5"/></svg>"#
        ),
    )
    .expect("background should write");
    service
    .import_background(&ticket.id, &source_background.to_string_lossy())
    .expect("background should import");
    ticket
}

fn print_archive_sample(
    service: &mut TicketService,
    ticket_id: &str,
    subticket_id: &str,
    local_date: &str,
) {
    service
    .print_one(ticket_id, subticket_id, local_date, None, None)
    .expect("ticket should print");
    service
    .adjust_counter(ticket_id, subticket_id, local_date, 7, "Import test")
    .expect("counter should adjust");
}

fn roundtrip_archive(service: &mut TicketService, ticket_id: &str) -> Ticket {
    let archive_path = service.data_dir().join("exports").join("entrance.tar.gz");
    let archive_path = service
    .export_ticket_archive(ticket_id, &archive_path.to_string_lossy())
    .expect("archive should export");
    service
    .import_ticket_archive(&archive_path)
    .expect("archive should import")
}

fn assert_imported_ticket(imported: &Ticket) {
    assert_eq!(imported.name, "Entrance");
    assert_eq!(imported.width_mm, 80.0);
    assert_eq!(imported.height_mm, 50.0);
    assert!(
        imported
        .background_svg_path
        .as_deref()
        .is_some_and(|path| { path.contains(&imported.id) && Path::new(path).is_file() })
    );
}

fn assert_imported_summary(
    imported_summary: &TicketSummary,
    summary: &TicketSummary,
    imported_id: &str,
) {
    assert_eq!(imported_summary.fields.len(), summary.fields.len());
    assert_eq!(imported_summary.subtickets.len(), summary.subtickets.len());
    assert_eq!(imported_summary.day_summaries[0].ticket_count, 1);
    assert_eq!(imported_summary.subtickets[0].next_number_today, 7);
    assert_eq!(imported_summary.recent_prints.len(), 1);
    assert!(
        imported_summary.recent_prints[0]
        .rendered_output_path
        .as_deref()
        .is_some_and(|path| path.contains(imported_id) && Path::new(path).is_file())
    );
}
