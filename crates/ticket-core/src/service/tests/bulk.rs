use super::*;

#[test]
fn one_ticket_bulk_renders_single_shared_output_for_all_records() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    install_bulk_template(&mut service, &ticket.id);
    let result = service
    .bulk_print(
        &ticket.id,
        "2026-08-02",
        &[
            BulkRequest {
                subticket_id: summary.subtickets[0].id.clone(),
                quantity: 2,
            },
            BulkRequest {
                subticket_id: summary.subtickets[1].id.clone(),
                quantity: 1,
            },
        ],
        BulkPrintOptions {
            count_only: false,
            single_ticket: true,
            label: Some("Class 7A"),
            sale_currency: None,
        },
    )
    .expect("one ticket bulk should succeed");

    assert_single_shared_output(&result.records, "Class 7A");
    let svg = rendered_bulk_svg(&result.records);
    assert!(svg.contains("Class 7A"));
    assert!(svg.contains("2x 10"));
    assert!(svg.contains("1x 7"));
    assert!(svg.contains("Total: 27"));
    assert!(svg.contains("<tspan"));
    assert!(!svg.contains(">3<"));
    assert!(!svg.contains(">27<"));

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should load");
    assert_eq!(after.day_summaries[0].ticket_count, 3);
    assert_eq!(after.subtickets[0].next_number_today, 3);
    assert_eq!(after.subtickets[1].next_number_today, 2);
    assert_bulk_history_entry(&after.recent_print_operations[0]);
}

#[test]
fn one_ticket_bulk_requires_user_created_template() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    let result = service.bulk_print(
        &ticket.id,
        "2026-08-02",
        &[BulkRequest {
                subticket_id: summary.subtickets[0].id.clone(),
                quantity: 2,
        }],
        BulkPrintOptions {
            count_only: false,
            single_ticket: true,
            label: Some("Class 7A"),
            sale_currency: None,
        },
    );

    assert!(
        result
        .expect_err("group ticket should require a template")
        .to_string()
        .contains("Group name and Tickets fields")
    );
}

#[test]
fn one_ticket_bulk_allows_an_unnamed_sale() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    install_bulk_template(&mut service, &ticket.id);

    let result = service
    .bulk_print(
        &ticket.id,
        "2026-08-02",
        &[BulkRequest {
                subticket_id: summary.subtickets[0].id.clone(),
                quantity: 2,
        }],
        BulkPrintOptions {
            count_only: false,
            single_ticket: true,
            label: None,
            sale_currency: None,
        },
    )
    .expect("an unnamed group ticket should print");

    for record in &result.records {
        assert_eq!(record.operation_label, None);
    }
    let svg = rendered_bulk_svg(&result.records);
    assert!(
        !svg.contains("Bulk ticket"),
        "an unnamed group must not print a fabricated name: {svg}"
    );
}

fn assert_single_shared_output(records: &[PrintRecord], label: &str) {
    assert_eq!(records.len(), 3);
    let output_path = records[0]
    .rendered_output_path
    .as_deref()
    .expect("bulk output should be rendered");
    for record in records {
        assert_eq!(record.status, "completed");
        assert_eq!(record.rendered_output_path.as_deref(), Some(output_path));
        assert_eq!(record.operation_label.as_deref(), Some(label));
    }
}

fn rendered_bulk_svg(records: &[PrintRecord]) -> String {
    let output_path = records[0]
    .rendered_output_path
    .as_deref()
    .expect("bulk output should be rendered");
    fs::read_to_string(output_path).expect("bulk SVG should be readable")
}

fn assert_bulk_history_entry(operation: &PrintHistoryEntry) {
    assert_eq!(operation.print_number, 1);
    assert!(operation.is_bulk);
    assert_eq!(operation.operation_label.as_deref(), Some("Class 7A"));
    assert_eq!(operation.status, "completed");
    assert_eq!(operation.ticket_count, 3);
    assert_eq!(operation.total_minor, 2_700);
    assert_eq!(operation.items.len(), 2);
    assert_eq!(operation.items[0].quantity, 2);
    assert_eq!(operation.items[0].ticket_numbers, vec![1, 2]);
    assert_eq!(operation.items[1].quantity, 1);
    assert_eq!(operation.items[1].ticket_numbers, vec![1]);
}

fn install_bulk_template(service: &mut TicketService, ticket_id: &str) {
    let recorded_at = now_iso();
    service
    .storage
    .replace_fields(
        ticket_id,
        &[
            test_field("classic", Some(DynamicKind::Number), "No. ", 5.0, 5.0),
            test_field("bulk", Some(DynamicKind::BulkName), "", 5.0, 5.0),
            test_field("bulk", Some(DynamicKind::BulkItems), "", 5.0, 15.0),
            test_field("bulk", Some(DynamicKind::BulkCount), "", 5.0, 25.0),
            test_field("bulk", Some(DynamicKind::BulkTotal), "", 45.0, 25.0),
        ],
        &recorded_at,
    )
    .expect("bulk fields should save");
}

fn test_field(
    template_kind: &str,
    dynamic_kind: Option<DynamicKind>,
    prefix: &str,
    x_mm: f64,
    y_mm: f64,
) -> TicketFieldInput {
    TicketFieldInput {
        id: String::new(),
        template_kind: template_kind.into(),
        field_kind: FieldKind::Dynamic,
        dynamic_kind,
        text: String::new(),
        x_mm,
        y_mm,
        width_mm: 30.0,
        height_mm: 8.0,
        font_family: "Noto Sans".into(),
        font_size_pt: 12.0,
        font_weight: 400,
        font_italic: false,
        horizontal_alignment: "center".into(),
        vertical_alignment: "center".into(),
        date_format: "date".into(),
        prefix: prefix.into(),
        suffix: String::new(),
        sort_order: 0,
    }
}
