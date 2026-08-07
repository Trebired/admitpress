use super::*;

#[test]
fn revert_last_sale_rolls_back_every_price_in_that_sale() {
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
                quantity: 3,
            },
            BulkRequest {
                subticket_id: summary.subtickets[1].id.clone(),
                quantity: 2,
            },
        ],
        BulkPrintOptions::default(),
    )
    .expect("sale should record");

    let sold = service
    .ticket_summary(&ticket.id, &local_date)
    .expect("summary should load");
    assert_eq!(sold.subtickets[0].next_number_today, 4);
    assert_eq!(sold.subtickets[1].next_number_today, 3);

    let reverted = service
    .revert_last_sale(&ticket.id, &local_date)
    .expect("last sale should revert");
    assert_eq!(reverted, 2);

    let after = service
    .ticket_summary(&ticket.id, &local_date)
    .expect("summary should load");
    assert_eq!(after.subtickets[0].next_number_today, 1);
    assert_eq!(after.subtickets[1].next_number_today, 1);
    assert_eq!(after.recent_prints.len(), 5, "print records are kept");
    assert_eq!(after.date_counter_adjustments.len(), 2);
}

#[test]
fn revert_last_sale_fails_when_there_is_no_sale() {
    let mut service = service();
    let local_date = today_local();
    let ticket = service
    .create_ticket("Entrance", 8.0, 5.0)
    .expect("ticket should be created");
    sample_ticket_summary(&mut service, &ticket.id, &local_date);

    assert!(service.revert_last_sale(&ticket.id, &local_date).is_err());
}
