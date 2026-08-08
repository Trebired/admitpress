use super::*;

const EMPTY_TICKET_MESSAGE: &str = "Add content in the editor before printing this ticket.";

fn blank_ticket_summary(service: &mut TicketService, local_date: &str) -> TicketSummary {
    let ticket = service
    .create_ticket("Blank", 8.0, 5.0)
    .expect("ticket should be created");
    let recorded_at = now_iso();
    service
    .storage
    .replace_subtickets(
        &ticket.id,
        &[SubticketInput {
                id: String::new(),
                name: "Standard".into(),
                amount_minor: 1_000,
                currency: String::new(),
                display_label: "Standard".into(),
                keyboard_shortcut: Some("1".into()),
                enabled: true,
                sort_order: 0,
        }],
        &recorded_at,
    )
    .expect("price variant should save");
    service
    .ticket_summary(&ticket.id, local_date)
    .expect("summary should load")
}

fn assert_empty_ticket_state(
    service: &mut TicketService,
    summary: &TicketSummary,
    local_date: &str,
) {
    let after = service
    .ticket_summary(&summary.ticket.id, local_date)
    .expect("summary should reload");
    assert_eq!(after.recent_prints.len(), 0);
    assert_eq!(after.subtickets[0].next_number_today, 1);
}

#[test]
fn print_one_rejects_ticket_without_printable_content() {
    let mut service = service();
    let local_date = "2026-08-02";
    let summary = blank_ticket_summary(&mut service, local_date);

    let result = service.print_one(
        &summary.ticket.id,
        &summary.subtickets[0].id,
        local_date,
        None,
        None,
    );

    assert!(
        result
        .expect_err("blank ticket should not print")
        .to_string()
        .contains(EMPTY_TICKET_MESSAGE)
    );
    assert_empty_ticket_state(&mut service, &summary, local_date);
}

#[test]
fn bulk_print_rejects_ticket_without_printable_content() {
    let mut service = service();
    let local_date = "2026-08-02";
    let summary = blank_ticket_summary(&mut service, local_date);

    let result = service.bulk_print(
        &summary.ticket.id,
        local_date,
        &[BulkRequest {
                subticket_id: summary.subtickets[0].id.clone(),
                quantity: 2,
        }],
        BulkPrintOptions::default(),
    );

    assert!(
        result
        .expect_err("blank ticket should not bulk print")
        .to_string()
        .contains(EMPTY_TICKET_MESSAGE)
    );
    assert_empty_ticket_state(&mut service, &summary, local_date);
}
