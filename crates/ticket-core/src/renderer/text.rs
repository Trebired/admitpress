use super::*;

pub(super) fn resolve_field_value(
    ticket: &Ticket,
    field: &TicketField,
    subticket: &Subticket,
    context: &RenderContext,
) -> String {
    let raw = match (&field.field_kind, &field.dynamic_kind) {
        (FieldKind::Static, _) => field.text.clone(),
        (FieldKind::Dynamic, Some(DynamicKind::Price)) => {
            let label = subticket.display_label.trim();
            let normalized = label.to_lowercase();
            if context.amount_minor == 0
            && (normalized.contains("free") || normalized.contains("zdarma"))
            {
                label.to_owned()
            } else {
                format_money(context.amount_minor, &context.currency)
            }
        }
        (FieldKind::Dynamic, Some(DynamicKind::Date)) => {
            format_date_value(field, &context.local_date, &context.local_datetime)
        }
        (FieldKind::Dynamic, Some(DynamicKind::Number)) => {
            format_number(ticket, context.allocated_number)
        }
        (FieldKind::Dynamic, Some(DynamicKind::BulkName)) => context.bulk_name.clone(),
        (FieldKind::Dynamic, Some(DynamicKind::BulkItems)) => context.bulk_items.clone(),
        (FieldKind::Dynamic, Some(DynamicKind::BulkCount)) => context.bulk_count.to_string(),
        (FieldKind::Dynamic, Some(DynamicKind::BulkTotal)) => {
            format_money(context.bulk_total_minor, &context.bulk_currency)
        }
        (FieldKind::Dynamic, None) => field.text.clone(),
        (FieldKind::Image, _) => String::new(),
    };
    format!("{}{}{}", field.prefix, raw, field.suffix)
}

const LINE_HEIGHT_FACTOR: f64 = 1.18;

pub(super) fn fitted_font_size_mm(field: &TicketField, value: &str) -> f64 {
    let requested = (field.font_size_pt * POINT_TO_MM).max(0.8);
    let line_count = text_lines(value).len();
    let max_for_height = (field.height_mm * 0.82 / text_block_units(line_count)).max(0.8);
    let mut font_size = requested.min(max_for_height);
    let available_width = (field.width_mm - 0.6).max(1.0);
    let estimated_width = estimate_text_width_mm(value, font_size, field.font_weight);
    if estimated_width > available_width {
        font_size *= available_width / estimated_width;
    }
    font_size.max(0.8)
}

pub(super) fn estimate_text_width_mm(value: &str, font_size_mm: f64, font_weight: i64) -> f64 {
    text_lines(value)
    .iter()
    .map(|line| estimate_text_line_width_mm(line, font_size_mm, font_weight))
    .fold(0.0, f64::max)
}

fn estimate_text_line_width_mm(value: &str, font_size_mm: f64, font_weight: i64) -> f64 {
    let weight_factor = if font_weight >= 700 { 1.08 } else { 1.0 };
    let units = value
    .chars()
    .map(|character| match character {
            ' ' => 0.32,
            '.' | ',' | ':' | ';' | '\'' | '"' | '!' | '|' => 0.26,
            'i' | 'j' | 'l' | 'I' | '1' => 0.32,
            'f' | 't' | 'r' | '-' | '/' => 0.40,
            'm' | 'w' | 'M' | 'W' | '@' => 0.82,
            '0'..='9' => 0.56,
            _ if character.is_ascii_uppercase() => 0.64,
            _ => 0.56,
    })
    .sum::<f64>();
    units * font_size_mm * weight_factor
}

pub(super) fn text_lines(value: &str) -> Vec<&str> {
    let lines = value.lines().collect::<Vec<_>>();
    if lines.is_empty() { vec![""] } else { lines }
}

pub(super) fn text_baseline(field: &TicketField, font_size_mm: f64, line_count: usize) -> f64 {
    let text_height = font_size_mm * text_block_units(line_count);
    match field.vertical_alignment.as_str() {
        "center" => field.y_mm + (field.height_mm - text_height) / 2.0 + font_size_mm * 0.85,
        "bottom" => field.y_mm + field.height_mm - text_height + font_size_mm * 0.8,
        _ => field.y_mm + font_size_mm * 0.85,
    }
}

pub(super) fn render_text_lines(lines: &[&str], text_x: f64, font_size_mm: f64) -> String {
    if lines.len() == 1 {
        return escape_xml(lines[0]);
    }
    let mut rendered = String::new();
    for (index, line) in lines.iter().enumerate() {
        let dy = if index == 0 {
            String::new()
        } else {
            format!(r#" dy="{}""#, font_size_mm * LINE_HEIGHT_FACTOR)
        };
        rendered.push_str(&format!(
                r#"<tspan x="{text_x}"{dy}>{line}</tspan>"#,
                line = escape_xml(line)
        ));
    }
    rendered
}

fn text_block_units(line_count: usize) -> f64 {
    1.0 + line_count.saturating_sub(1) as f64 * LINE_HEIGHT_FACTOR
}

pub(super) fn svg_font_family(font_family: &str) -> String {
    let trimmed = font_family.trim();
    if trimmed.is_empty() || trimmed == "Sans Serif" {
        "Noto Sans".to_owned()
    } else {
        trimmed.to_owned()
    }
}

pub(super) fn format_date_value(
    field: &TicketField,
    local_date: &str,
    local_datetime: &str,
) -> String {
    match field.date_format.as_str() {
        "date_time" => format!(
            "{} {}",
            format_date(local_date),
            format_time(local_datetime, false)
        ),
        "full_date_time" => {
            format!(
                "{} {}",
                format_date(local_date),
                format_time(local_datetime, true)
            )
        }
        "iso_date" => local_date.to_owned(),
        "iso_date_time" => format!("{} {}", local_date, format_time(local_datetime, true)),
        "time" => format_time(local_datetime, false),
        _ => format_date(local_date),
    }
}

pub(super) fn format_time(local_datetime: &str, include_seconds: bool) -> String {
    DateTime::parse_from_rfc3339(local_datetime)
    .map(|datetime| {
            if include_seconds {
                datetime.format("%H:%M:%S").to_string()
            } else {
                datetime.format("%H:%M").to_string()
            }
    })
    .unwrap_or_else(|_| {
            if include_seconds {
                "00:00:00".to_owned()
            } else {
                "00:00".to_owned()
            }
    })
}

pub(crate) fn escape_xml(value: &str) -> String {
    value
    .replace('&', "&amp;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
    .replace('"', "&quot;")
    .replace('\'', "&apos;")
}
