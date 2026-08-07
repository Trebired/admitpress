use super::*;

fn price_field() -> TicketFieldInput {
    TicketFieldInput {
        id: String::new(),
        template_kind: "classic".into(),
        field_kind: FieldKind::Dynamic,
        dynamic_kind: Some(DynamicKind::Price),
        text: String::new(),
        x_mm: 5.0,
        y_mm: 5.0,
        width_mm: 30.0,
        height_mm: 8.0,
        font_family: "Noto Sans".into(),
        font_size_pt: 12.0,
        font_weight: 400,
        horizontal_alignment: "center".into(),
        vertical_alignment: "center".into(),
        date_format: "date".into(),
        prefix: String::new(),
        suffix: String::new(),
        sort_order: 0,
    }
}

#[test]
fn main_currency_defaults_from_locale_and_persists() {
    let mut service = service();
    assert_eq!(service.main_currency(), "");

    let resolved = service
    .ensure_main_currency("cs_CZ")
    .expect("locale default should resolve");
    assert_eq!(resolved, "CZK");
    assert_eq!(service.main_currency(), "CZK");

    service
    .ensure_main_currency("en_US")
    .expect("existing main currency should be kept");
    assert_eq!(service.main_currency(), "CZK");
}

#[test]
fn main_currency_falls_back_to_dollars_when_locale_has_no_region() {
    let mut service = service();
    let resolved = service
    .ensure_main_currency("xx")
    .expect("fallback should resolve");
    assert_eq!(resolved, "USD");
}

#[test]
fn set_main_currency_rejects_unknown_codes() {
    let mut service = service();
    assert!(service.set_main_currency("NOTREAL").is_err());
}

#[test]
fn print_one_converts_amount_into_chosen_sale_currency() {
    let mut service = service();
    service.set_development_mode(true).ok();
    service
    .set_main_currency("CZK")
    .expect("main currency should save");
    service
    .storage
    .store_exchange_rate("CZK", "EUR", 0.04, &now_iso())
    .expect("exchange rate cache should seed");

    let ticket = service
    .create_ticket("Zoo", 5.0, 7.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-06");
    let subticket = &summary.subtickets[0];
    assert_eq!(subticket.amount_minor, 1_000);

    let result = service
    .print_one(&ticket.id, &subticket.id, "2026-08-06", Some("EUR"), None)
    .expect("print should succeed");

    assert_eq!(result.record.sold_currency.as_deref(), Some("EUR"));
    assert_eq!(result.record.sold_amount_minor, Some(40));
}

#[test]
fn cached_convert_amount_returns_none_without_a_cached_rate() {
    let service = service();
    assert_eq!(service.cached_convert_amount(1_000, "CZK", "EUR").unwrap(), None);
}

#[test]
fn cached_convert_amount_uses_seeded_cache() {
    let service = service();
    service
    .storage
    .store_exchange_rate("CZK", "EUR", 0.04, &now_iso())
    .expect("exchange rate cache should seed");
    assert_eq!(
        service.cached_convert_amount(1_000, "CZK", "EUR").unwrap(),
        Some(40)
    );
}

#[test]
fn cached_convert_amount_ignores_a_rate_older_than_the_cache_window() {
    let service = service();
    let stale = (Local::now() - chrono::Duration::hours(7)).to_rfc3339();
    service
    .storage
    .store_exchange_rate("CZK", "EUR", 0.04, &stale)
    .expect("exchange rate cache should seed");
    assert_eq!(service.cached_convert_amount(1_000, "CZK", "EUR").unwrap(), None);
}

#[test]
fn cached_convert_amount_keeps_a_rate_inside_the_cache_window() {
    let service = service();
    let recent = (Local::now() - chrono::Duration::hours(5)).to_rfc3339();
    service
    .storage
    .store_exchange_rate("CZK", "EUR", 0.04, &recent)
    .expect("exchange rate cache should seed");
    assert_eq!(
        service.cached_convert_amount(1_000, "CZK", "EUR").unwrap(),
        Some(40)
    );
}

#[test]
fn cached_convert_amount_same_currency_is_identity() {
    let service = service();
    assert_eq!(
        service.cached_convert_amount(1_234, "CZK", "CZK").unwrap(),
        Some(1_234)
    );
}

fn printed_price_text(
    service: &mut TicketService,
    ticket_id: &str,
    subticket_id: &str,
    sale_currency: Option<&str>,
) -> String {
    let result = service
    .print_one(ticket_id, subticket_id, "2026-08-06", sale_currency, None)
    .expect("print should succeed");
    std::fs::read_to_string(
        result
        .record
        .rendered_output_path
        .as_deref()
        .expect("development print writes an svg"),
    )
    .expect("rendered svg should be readable")
}

#[test]
fn printed_ticket_shows_the_symbol_for_the_main_currency() {
    let mut service = service();
    service.set_development_mode(true).ok();
    service
    .set_main_currency("CZK")
    .expect("main currency should save");

    let ticket = service
    .create_ticket("Zoo", 5.0, 7.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-06");
    let subticket_id = summary.subtickets[0].id.clone();
    assert_eq!(summary.subtickets[0].currency, "");
    service
    .storage
    .replace_fields(&ticket.id, &[price_field()], &now_iso())
    .expect("price field should save");

    let svg = printed_price_text(&mut service, &ticket.id, &subticket_id, None);
    assert!(
        svg.contains("10 Kč"),
        "the main currency should print its symbol: {svg}"
    );
}

#[test]
fn printed_ticket_shows_the_symbol_for_a_foreign_sale_currency() {
    let mut service = service();
    service.set_development_mode(true).ok();
    service
    .set_main_currency("CZK")
    .expect("main currency should save");
    service
    .storage
    .store_exchange_rate("CZK", "EUR", 0.04, &now_iso())
    .expect("exchange rate cache should seed");

    let ticket = service
    .create_ticket("Zoo", 5.0, 7.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-06");
    let subticket_id = summary.subtickets[0].id.clone();
    service
    .storage
    .replace_fields(&ticket.id, &[price_field()], &now_iso())
    .expect("price field should save");

    let svg = printed_price_text(&mut service, &ticket.id, &subticket_id, Some("EUR"));
    assert!(
        svg.contains("0,40 €"),
        "a foreign sale currency should print its symbol: {svg}"
    );
}

#[test]
fn print_one_records_the_main_currency_when_no_sale_currency_is_chosen() {
    let mut service = service();
    service.set_development_mode(true).ok();
    service
    .set_main_currency("CZK")
    .expect("main currency should save");
    let ticket = service
    .create_ticket("Zoo", 5.0, 7.0)
    .expect("ticket should be created");
    let summary = sample_ticket_summary(&mut service, &ticket.id, "2026-08-06");
    let subticket = &summary.subtickets[0];

    let result = service
    .print_one(&ticket.id, &subticket.id, "2026-08-06", None, None)
    .expect("print should succeed");

    assert_eq!(result.record.sold_currency.as_deref(), Some("CZK"));
    assert_eq!(result.record.sold_amount_minor, Some(1_000));
}
