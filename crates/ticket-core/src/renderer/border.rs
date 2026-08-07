use super::*;

pub(super) fn render_ticket_border(ticket: &Ticket) -> String {
    let style = ticket.border_style.as_str();
    if style == "none" || ticket.border_width_mm <= 0.0 {
        return String::new();
    }

    let stroke_width = ticket
    .border_width_mm
    .min(ticket.width_mm / 2.0)
    .min(ticket.height_mm / 2.0)
    .max(0.1);
    let inset = stroke_width / 2.0;
    let rect_width = (ticket.width_mm - stroke_width).max(0.0);
    let rect_height = (ticket.height_mm - stroke_width).max(0.0);
    let (linecap, dasharray) = match style {
        "dashed" => (
            "butt",
            format!(
                r#" stroke-dasharray="{} {}""#,
                stroke_width * 8.0,
                stroke_width * 4.0
            ),
        ),
        "dotted" => (
            "round",
            format!(
                r#" stroke-dasharray="{} {}""#,
                stroke_width * 0.1,
                stroke_width * 3.0
            ),
        ),
        _ => ("butt", String::new()),
    };

    format!(
        r#"
        <rect
        x="{inset}" y="{inset}" width="{rect_width}" height="{rect_height}" fill="none" stroke="black" stroke-width="{stroke_width}"
        stroke-linecap="{linecap}"{dasharray}/>"#
    )
}
