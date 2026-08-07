use super::*;

fn field_inputs(summary: &TicketSummary) -> Vec<TicketFieldInput> {
    summary
    .fields
    .iter()
    .map(|field| TicketFieldInput {
            id: field.id.clone(),
            template_kind: field.template_kind.clone(),
            field_kind: field.field_kind.clone(),
            dynamic_kind: field.dynamic_kind.clone(),
            text: field.text.clone(),
            x_mm: field.x_mm,
            y_mm: field.y_mm,
            width_mm: field.width_mm,
            height_mm: field.height_mm,
            font_family: field.font_family.clone(),
            font_size_pt: field.font_size_pt,
            font_weight: field.font_weight,
            horizontal_alignment: field.horizontal_alignment.clone(),
            vertical_alignment: field.vertical_alignment.clone(),
            date_format: field.date_format.clone(),
            prefix: field.prefix.clone(),
            suffix: field.suffix.clone(),
            sort_order: field.sort_order,
    })
    .collect()
}

fn subticket_inputs(summary: &TicketSummary) -> Vec<SubticketInput> {
    summary
    .subtickets
    .iter()
    .map(|subticket| SubticketInput {
            id: subticket.id.clone(),
            name: subticket.name.clone(),
            amount_minor: subticket.amount_minor,
            currency: subticket.currency.clone(),
            display_label: subticket.display_label.clone(),
            keyboard_shortcut: subticket.keyboard_shortcut.clone(),
            enabled: subticket.enabled,
            sort_order: subticket.sort_order,
    })
    .collect()
}

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
