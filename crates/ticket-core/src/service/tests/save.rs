use super::*;

fn save_summary(service: &mut TicketService, summary: &TicketSummary) {
    service
    .save_ticket(
        &summary.ticket.id,
        &summary.ticket.name,
        summary.ticket.width_mm / 10.0,
        summary.ticket.height_mm / 10.0,
        summary.ticket.printer_queue.as_deref(),
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
fn editor_save_preserves_printing_day_data() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    let printed = service
    .print_one(&ticket.id, &summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("ticket should print");

    save_summary(&mut service, &summary);

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should reload");
    assert_eq!(after.subtickets[0].id, summary.subtickets[0].id);
    assert_eq!(after.subtickets[0].next_number_today, printed.next_number);
    assert_eq!(after.day_summaries[0].ticket_count, 1);
    assert_eq!(
        after.day_summaries[0].subtickets[0].subticket_id,
        summary.subtickets[0].id
    );
    assert_eq!(after.recent_prints.len(), 1);
}

#[test]
fn removing_used_price_variant_preserves_day_history() {
    let mut service = service();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-02");
    service
    .print_one(&ticket.id, &summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("ticket should print");
    let fields = field_inputs(&summary);
    let mut subtickets = subticket_inputs(&summary);
    let removed_id = subtickets.remove(0).id;

    service
    .save_ticket(
        &ticket.id,
        "Entrance",
        8.0,
        5.0,
        None,
        &summary.ticket.border_style,
        summary.ticket.border_width_mm,
        summary.ticket.dpi,
        summary.ticket.number_start,
        summary.ticket.number_increment,
        summary.ticket.number_padding,
        &summary.ticket.number_prefix,
        &summary.ticket.number_suffix,
        &fields,
        &subtickets,
    )
    .expect("ticket should save");

    let after = service
    .ticket_summary(&ticket.id, "2026-08-02")
    .expect("summary should reload");
    assert!(
        after
        .subtickets
        .iter()
        .all(|subticket| subticket.id != removed_id)
    );
    assert_eq!(after.day_summaries[0].ticket_count, 1);
    assert_eq!(
        after.day_summaries[0].subtickets[0].subticket_id,
        removed_id
    );
}
