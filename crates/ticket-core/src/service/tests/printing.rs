use super::*;

#[test]
fn preview_ticket_prefers_png_when_rasterizer_is_available() {
    if Command::new("rsvg-convert")
    .arg("--version")
    .output()
    .is_err()
    && Command::new("inkscape").arg("--version").output().is_err()
    {
        return;
    }

    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");

    let preview_url = service
    .preview_ticket(&ticket.id, &summary.subtickets[0].id, "2026-08-02", 1)
    .expect("preview should render");
    let preview_path = PathBuf::from(decode_file_path(&preview_url));
    let preview_bytes = fs::read(&preview_path).expect("preview PNG should be readable");

    assert!(preview_url.ends_with(".png"));
    assert!(preview_path.is_file());
    assert!(preview_bytes.starts_with(b"\x89PNG\r\n\x1A\n"));
    assert!(preview_path.with_extension("svg").is_file());
}

#[test]
fn creates_ticket_without_default_fields_or_price_variants() {
    let mut service = service();
    let ticket = service
    .create_ticket("Sample", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should load");
    assert_eq!(summary.fields.len(), 0);
    assert_eq!(summary.subtickets.len(), 0);
    assert_eq!(summary.next_number_today, 1);
    assert_eq!(summary.day_summaries[0].ticket_count, 0);
    assert_eq!(summary.day_summaries[0].currency, "");
}

#[test]
fn keeps_daily_counters_per_subticket() {
    let mut service = service();
    let local_date = today_local();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, &local_date);
    let first = service
    .print_one(&ticket.id, &summary.subtickets[0].id, &local_date, None, None)
    .expect("first ticket should print to development output");
    let second = service
    .print_one(&ticket.id, &summary.subtickets[1].id, &local_date, None, None)
    .expect("second ticket should print to development output");
    assert_eq!(first.record.allocated_number, 1);
    assert_eq!(second.record.allocated_number, 1);
    assert_eq!(second.next_number, 2);
    let after = service
    .ticket_summary(&ticket.id, &local_date)
    .expect("summary should load");
    assert_eq!(after.subtickets[0].next_number_today, 2);
    assert_eq!(after.subtickets[1].next_number_today, 2);
}

#[test]
fn separates_counters_by_date() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    service
    .print_one(&ticket.id, &summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("ticket should print");
    let next_day = service
    .ticket_summary(&ticket.id, "2026-08-03")
    .expect("next day should load");
    assert_eq!(next_day.subtickets[0].next_number_today, 1);
}

#[test]
fn bulk_reserves_per_subticket_ranges() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
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
        BulkPrintOptions::default(),
    )
    .expect("bulk print should succeed");
    assert_eq!(result.first_number, Some(1));
    assert_eq!(result.final_number, Some(1));
    assert_eq!(result.next_number, 3);
    assert_eq!(
        result
        .next_numbers
        .iter()
        .find(|state| state.subticket_id == summary.subtickets[0].id)
        .map(|state| state.next_number),
        Some(3)
    );
    assert_eq!(
        result
        .next_numbers
        .iter()
        .find(|state| state.subticket_id == summary.subtickets[1].id)
        .map(|state| state.next_number),
        Some(2)
    );
}

#[test]
fn development_print_succeeds_without_selected_printer() {
    let mut service = service();
    service
    .set_selected_printer_queue(None)
    .expect("printer queue should clear");
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");

    let result = service
    .print_one(&ticket.id, &summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("development print should render");

    assert_eq!(result.record.status, "completed");
    assert_eq!(result.record.printer_name, None);
    assert!(result.output_path.ends_with(".svg"));
}

#[test]
fn failed_physical_print_keeps_allocated_number() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    service
    .set_development_mode(false)
    .expect("physical mode should be enabled");
    service
    .set_selected_printer_queue(None)
    .expect("printer queue should clear");

    let result = service.print_one(&ticket.id, &summary.subtickets[0].id, "2026-08-02", None, None);
    assert!(result.is_err());

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should load");
    assert_eq!(after.subtickets[0].next_number_today, 2);
    assert_eq!(after.recent_prints.len(), 1);
    assert_eq!(after.recent_prints[0].status, "failed");
    assert_eq!(after.recent_prints[0].allocated_number, 1);
    assert_eq!(
        after.recent_prints[0].failure_reason.as_deref(),
        Some("no printer queue is selected in settings")
    );
}

#[test]
fn counter_adjustment_does_not_delete_history() {
    let mut service = service();
    let local_date = today_local();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, &local_date);
    service
    .print_one(&ticket.id, &summary.subtickets[0].id, &local_date, None, None)
    .expect("ticket should print");
    service
    .adjust_counter(
        &ticket.id,
        &summary.subtickets[0].id,
        &local_date,
        1,
        "Paper jam",
    )
    .expect("counter should adjust");
    let after = service
    .ticket_summary(&ticket.id, &local_date)
    .expect("summary should load");
    assert_eq!(after.subtickets[0].next_number_today, 1);
    assert_eq!(after.recent_prints.len(), 1);
}

#[test]
fn day_summary_counts_tickets_and_money_then_resets() {
    let mut service = service();
    let local_date = today_local();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, &local_date);
    service
    .bulk_print(
        &ticket.id,
        &local_date,
        &[
            BulkRequest {
                subticket_id: summary.subtickets[0].id.clone(),
                quantity: 2,
            },
            BulkRequest {
                subticket_id: summary.subtickets[1].id.clone(),
                quantity: 3,
            },
        ],
        BulkPrintOptions::default(),
    )
    .expect("bulk print should succeed");

    let after = service
    .ticket_summary(&ticket.id, &local_date)
    .expect("summary should load");
    assert_eq!(after.day_summaries[0].ticket_count, 5);
    assert_eq!(after.day_summaries[0].total_minor, 4_100);
    assert_eq!(after.day_summaries[0].subtickets[0].quantity, 2);
    assert_eq!(after.day_summaries[0].subtickets[1].quantity, 3);

    service
    .reset_day(&ticket.id, &local_date)
    .expect("day should reset");
    let reset = service
    .ticket_summary(&ticket.id, &local_date)
    .expect("summary should reload");
    assert_eq!(reset.day_summaries[0].ticket_count, 0);
    assert_eq!(reset.subtickets[0].next_number_today, 1);
    assert_eq!(reset.recent_prints.len(), 0);
}

#[test]
fn non_current_day_with_sales_is_locked() {
    let mut service = service();
    let locked_date = "2000-01-01";
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, locked_date);
    service
    .print_one(&ticket.id, &summary.subtickets[0].id, locked_date, None, None)
    .expect("first non-current sale can create day data");

    let second = service.print_one(&ticket.id, &summary.subtickets[0].id, locked_date, None, None);
    assert!(
        second
        .expect_err("sold non-current day should be locked")
        .to_string()
        .contains("already has data")
    );
    let reset = service.reset_day(&ticket.id, locked_date);
    assert!(
        reset
        .expect_err("sold non-current day should not reset")
        .to_string()
        .contains("already has data")
    );
}

#[test]
fn count_only_bulk_records_sales_without_rendering_output() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    let result = service
    .bulk_print(
        &ticket.id,
        "2026-08-02",
        &[BulkRequest {
                subticket_id: summary.subtickets[0].id.clone(),
                quantity: 3,
        }],
        BulkPrintOptions {
            count_only: true,
            single_ticket: false,
            label: Some("Morning school group"),
            sale_currency: None,
        },
    )
    .expect("count only bulk should succeed");

    assert_eq!(result.records.len(), 3);
    for record in &result.records {
        assert_eq!(record.status, "completed");
        assert_eq!(record.rendered_output_path, None);
        assert_eq!(record.printer_job_id, None);
        assert_eq!(
            record.operation_label.as_deref(),
            Some("Morning school group")
        );
    }

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should load");
    assert_eq!(after.day_summaries[0].ticket_count, 3);
    assert_eq!(after.subtickets[0].next_number_today, 4);
}
