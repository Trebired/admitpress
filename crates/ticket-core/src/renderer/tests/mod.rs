use super::*;

mod dynamic;
mod formatting;
mod image;

fn rendered_font_size(svg: &str) -> f64 {
    let marker = r#"font-size=""#;
    let start = svg.find(marker).expect("font size should exist") + marker.len();
    let end = svg[start..]
    .find('"')
    .expect("font size attribute should end");
    svg[start..start + end]
    .parse::<f64>()
    .expect("font size should parse")
}

fn sample_ticket(name: &str, width_mm: f64, height_mm: f64) -> Ticket {
    Ticket {
        id: "ticket".into(),
        name: name.into(),
        width_mm,
        height_mm,
        background_svg_path: None,
        printer_queue: None,
        border_style: "none".into(),
        border_width_mm: 0.2,
        dpi: 203,
        number_start: 1,
        number_increment: 1,
        number_padding: 0,
        number_prefix: String::new(),
        number_suffix: String::new(),
        recorded_at: "2026-08-02T22:00:00+02:00".into(),
        updated_at: "2026-08-02T22:00:00+02:00".into(),
    }
}

fn sample_subticket(ticket: &Ticket) -> Subticket {
    Subticket {
        id: "standard".into(),
        ticket_id: ticket.id.clone(),
        name: "Standard".into(),
        amount_minor: 1_000,
        currency: String::new(),
        display_label: "Standard".into(),
        keyboard_shortcut: None,
        enabled: true,
        sort_order: 0,
        recorded_at: ticket.recorded_at.clone(),
        updated_at: ticket.updated_at.clone(),
    }
}

fn sample_context(subticket: &Subticket, allocated_number: i64) -> RenderContext {
    RenderContext {
        local_date: "2026-08-02".into(),
        local_datetime: "2026-08-02T14:05:06+02:00".into(),
        allocated_number,
        amount_minor: subticket.amount_minor,
        currency: subticket.currency.clone(),
        subticket_name: subticket.name.clone(),
        bulk_name: String::new(),
        bulk_items: String::new(),
        bulk_count: 0,
        bulk_total_minor: 0,
        bulk_currency: String::new(),
    }
}

fn field_base(ticket: &Ticket, id: &str, field_kind: FieldKind) -> TicketField {
    TicketField {
        id: id.into(),
        ticket_id: ticket.id.clone(),
        template_kind: "classic".into(),
        field_kind,
        dynamic_kind: None,
        text: String::new(),
        x_mm: 5.0,
        y_mm: 5.0,
        width_mm: 20.0,
        height_mm: 4.0,
        font_family: "Sans Serif".into(),
        font_size_pt: 12.0,
        font_weight: 400,
        horizontal_alignment: "left".into(),
        vertical_alignment: "top".into(),
        date_format: "date".into(),
        prefix: String::new(),
        suffix: String::new(),
        sort_order: 0,
        recorded_at: ticket.recorded_at.clone(),
        updated_at: ticket.updated_at.clone(),
    }
}
