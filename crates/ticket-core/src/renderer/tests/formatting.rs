use super::*;

#[test]
fn formats_money_without_currency_and_known_currencies() {
    assert_eq!(format_money(1_000, ""), "10");
    assert_eq!(format_money(123, "CZK"), "1,23 Kč");
    assert_eq!(format_money(100_000_000, "CZK"), "1 000 000 Kč");
}

#[test]
fn formats_date_field_variants() {
    let ticket = sample_ticket("Date", 40.0, 20.0);
    let mut field = field_base(&ticket, "date", FieldKind::Dynamic);
    field.dynamic_kind = Some(DynamicKind::Date);
    field.width_mm = 40.0;
    field.height_mm = 8.0;
    field.horizontal_alignment = "center".into();
    field.vertical_alignment = "center".into();
    let local_date = "2026-08-03";
    let local_datetime = "2026-08-03T14:05:06+02:00";
    assert_eq!(
        format_date_value(&field, local_date, local_datetime),
        "03.08.2026"
    );
    field.date_format = "date_time".into();
    assert_eq!(
        format_date_value(&field, local_date, local_datetime),
        "03.08.2026 14:05"
    );
    field.date_format = "full_date_time".into();
    assert_eq!(
        format_date_value(&field, local_date, local_datetime),
        "03.08.2026 14:05:06"
    );
    field.date_format = "iso_date_time".into();
    assert_eq!(
        format_date_value(&field, local_date, local_datetime),
        "2026-08-03 14:05:06"
    );
    field.date_format = "time".into();
    assert_eq!(
        format_date_value(&field, local_date, local_datetime),
        "14:05"
    );
}
