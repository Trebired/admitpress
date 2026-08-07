use super::*;

#[test]
fn renders_dynamic_ticket_values() {
    let ticket = sample_ticket("Sample", 80.0, 50.0);
    let subticket = sample_subticket(&ticket);
    let fields = vec![
        dynamic_field(&ticket, "price", DynamicKind::Price, "Price: ", 5.0),
        dynamic_field(&ticket, "date", DynamicKind::Date, "Date: ", 15.0),
        dynamic_field(&ticket, "number", DynamicKind::Number, "No. ", 25.0),
    ];
    let context = sample_context(&subticket, 42);

    let svg =
    render_ticket_svg(&ticket, &fields, &subticket, &context).expect("ticket should render");
    assert!(svg.contains(r#"width="80mm""#));
    assert!(svg.contains(r#"height="50mm""#));
    assert!(svg.contains("Price: 10"));
    assert!(svg.contains("Date: 02.08.2026"));
    assert!(svg.contains("No. 42"));
}

#[test]
fn formats_padded_number() {
    let mut ticket = sample_ticket("Ticket", 80.0, 50.0);
    ticket.number_prefix = "A-".into();
    ticket.number_padding = 4;
    assert_eq!(format_number(&ticket, 42), "A-0042");
}

#[test]
fn formats_unpadded_number_by_default() {
    let ticket = sample_ticket("Ticket", 80.0, 50.0);
    assert_eq!(format_number(&ticket, 42), "42");
}

#[test]
fn renders_whole_ticket_border() {
    let mut ticket = sample_ticket("Bordered", 70.0, 50.0);
    ticket.border_style = "dotted".into();
    ticket.border_width_mm = 0.4;
    let subticket = sample_subticket(&ticket);
    let context = sample_context(&subticket, 1);

    let svg = render_ticket_svg(&ticket, &[], &subticket, &context).expect("ticket should render");

    assert!(svg.contains(r#"stroke-width="0.4""#));
    assert!(svg.contains(r#"stroke-linecap="round""#));
    assert!(svg.contains(r#"stroke-dasharray=""#));
}

fn dynamic_field(
    ticket: &Ticket,
    id: &str,
    dynamic_kind: DynamicKind,
    prefix: &str,
    y_mm: f64,
) -> TicketField {
    let mut field = field_base(ticket, id, FieldKind::Dynamic);
    field.dynamic_kind = Some(dynamic_kind);
    field.prefix = prefix.into();
    field.y_mm = y_mm;
    field.width_mm = 70.0;
    field.height_mm = 8.0;
    field
}
