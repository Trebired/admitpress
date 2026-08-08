use super::*;

#[test]
fn defaults_to_activity_sort_with_no_setting() {
    let service = service();
    assert_eq!(service.home_sort_mode().expect("mode should read"), "activity");
    assert!(service.home_ticket_order().expect("order should read").is_empty());
}

#[test]
fn rejects_an_unknown_sort_mode() {
    let mut service = service();
    assert!(service.set_home_sort_mode("random").is_err());
}

#[test]
fn persists_the_chosen_sort_mode() {
    let mut service = service();
    service
    .set_home_sort_mode("alphabetical")
    .expect("mode should save");
    assert_eq!(
        service.home_sort_mode().expect("mode should read"),
        "alphabetical"
    );
}

#[test]
fn alphabetical_mode_ignores_case_and_creation_order() {
    let mut service = service();
    service
    .create_ticket("Zebra Crossing", 5.0, 7.0)
    .expect("ticket should be created");
    service
    .create_ticket("apple stand", 5.0, 7.0)
    .expect("ticket should be created");
    service
    .set_home_sort_mode("alphabetical")
    .expect("mode should save");

    let state = service.application_state().expect("state should load");
    let names: Vec<&str> = state
    .tickets
    .iter()
    .map(|summary| summary.ticket.name.as_str())
    .collect();
    assert_eq!(names, vec!["apple stand", "Zebra Crossing"]);
}

#[test]
fn created_mode_puts_the_newest_ticket_first() {
    let mut service = service();
    service
    .create_ticket("First", 5.0, 7.0)
    .expect("ticket should be created");
    service
    .create_ticket("Second", 5.0, 7.0)
    .expect("ticket should be created");
    service
    .set_home_sort_mode("created")
    .expect("mode should save");

    let state = service.application_state().expect("state should load");
    let names: Vec<&str> = state
    .tickets
    .iter()
    .map(|summary| summary.ticket.name.as_str())
    .collect();
    assert_eq!(names, vec!["Second", "First"]);
}

#[test]
fn activity_mode_ranks_the_most_recently_printed_ticket_first() {
    let mut service = service();
    let quiet = service
    .create_ticket("Quiet ticket", 5.0, 7.0)
    .expect("ticket should be created");
    let busy = service
    .create_ticket("Busy ticket", 5.0, 7.0)
    .expect("ticket should be created");
    let quiet_summary = sample_ticket_summary(&mut service, &quiet.id, "2026-08-02");
    let busy_summary = sample_ticket_summary(&mut service, &busy.id, "2026-08-02");

    service
    .print_one(&quiet.id, &quiet_summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("quiet ticket should print first");
    service
    .print_one(&busy.id, &busy_summary.subtickets[0].id, "2026-08-02", None, None)
    .expect("busy ticket should print after");

    let state = service.application_state().expect("state should load");
    let names: Vec<&str> = state
    .tickets
    .iter()
    .map(|summary| summary.ticket.name.as_str())
    .collect();
    assert_eq!(names, vec!["Busy ticket", "Quiet ticket"]);
}

#[test]
fn manual_mode_follows_the_saved_order_and_appends_unlisted_tickets() {
    let mut service = service();
    let first = service
    .create_ticket("First", 5.0, 7.0)
    .expect("ticket should be created");
    let second = service
    .create_ticket("Second", 5.0, 7.0)
    .expect("ticket should be created");
    service
    .create_ticket("Third", 5.0, 7.0)
    .expect("ticket should be created");

    service
    .set_home_sort_mode("manual")
    .expect("mode should save");
    service
    .set_home_ticket_order(&[second.id.clone(), first.id.clone()])
    .expect("order should save");

    let state = service.application_state().expect("state should load");
    let names: Vec<&str> = state
    .tickets
    .iter()
    .map(|summary| summary.ticket.name.as_str())
    .collect();
    assert_eq!(names, vec!["Second", "First", "Third"]);
}
