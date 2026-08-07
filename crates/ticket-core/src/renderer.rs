use crate::models::{DynamicKind, FieldKind, RenderContext, Subticket, Ticket, TicketField};
use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use chrono::DateTime;
use iso_currency::Currency;
use std::fs;
use std::path::Path;

mod assets;
mod border;
mod text;

#[cfg(test)]
mod tests;

use assets::*;
use border::*;
pub(crate) use text::escape_xml as escape_xml_value;
use text::*;

const POINT_TO_MM: f64 = 25.4 / 72.0;

pub fn render_ticket_svg(
    ticket: &Ticket,
    fields: &[TicketField],
    subticket: &Subticket,
    context: &RenderContext,
) -> Result<String> {
    let background = ticket
    .background_svg_path
    .as_deref()
    .map(|path| read_svg_image(path, ticket.width_mm, ticket.height_mm))
    .transpose()?
    .unwrap_or_default();

    let mut clip_paths = String::new();
    for (index, field) in fields.iter().enumerate() {
        clip_paths.push_str(&format!(
                r#"
                <clipPath id="field-clip-{index}">
                <rect x="{x}" y="{y}" width="{width}" height="{height}"/>
                </clipPath>"#,
                x = field.x_mm,
                y = field.y_mm,
                width = field.width_mm,
                height = field.height_mm,
        ));
    }

    let mut svg = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <svg xmlns="http://www.w3.org/2000/svg" width="{width}mm" height="{height}mm" viewBox="0 0 {width} {height}">
        <defs>{clip_paths}
        </defs>
        <rect x="0" y="0" width="{width}" height="{height}" fill="white"/>
        <g id="background">{background}</g>
        <g id="ticket-fields">"#,
        width = ticket.width_mm,
        height = ticket.height_mm,
        clip_paths = clip_paths,
        background = background,
    );

    for (index, field) in fields.iter().enumerate() {
        if field.field_kind == FieldKind::Image {
            let image = read_ticket_image(&field.text)?;
            svg.push_str(&format!(
                    r#"
                    <g clip-path="url(#field-clip-{index})">
                    <image x="{x}" y="{y}" width="{width}" height="{height}" preserveAspectRatio="xMidYMid meet" href="{image}"/>
                    </g>"#,
                    index = index,
                    x = field.x_mm,
                    y = field.y_mm,
                    width = field.width_mm,
                    height = field.height_mm,
                    image = image,
            ));
            continue;
        }

        let value = resolve_field_value(ticket, field, subticket, context);
        let x = field.x_mm;
        let font_size_mm = fitted_font_size_mm(field, &value);
        let lines = text_lines(&value);
        let y = text_baseline(field, font_size_mm, lines.len());
        let anchor = match field.horizontal_alignment.as_str() {
            "center" => "middle",
            "right" => "end",
            _ => "start",
        };
        let text_x = match field.horizontal_alignment.as_str() {
            "center" => x + field.width_mm / 2.0,
            "right" => x + field.width_mm,
            _ => x,
        };
        let rendered_text = render_text_lines(&lines, text_x, font_size_mm);
        svg.push_str(&format!(
                r#"
                <g clip-path="url(#field-clip-{index})">
                <text
                x="{text_x}" y="{y}" text-anchor="{anchor}" font-family="{family}" font-size="{font_size_mm}" font-weight="{weight}"
                fill="black">{rendered_text}</text>
                </g>"#,
                index = index,
                text_x = text_x,
                y = y,
                anchor = anchor,
                family = escape_xml(&svg_font_family(&field.font_family)),
                font_size_mm = font_size_mm,
                weight = field.font_weight,
                rendered_text = rendered_text,
        ));
    }

    svg.push_str("\n</g>");
    svg.push_str(&render_ticket_border(ticket));
    svg.push_str("\n</svg>\n");
    Ok(svg)
}

pub fn save_svg(path: &Path, svg: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, svg).with_context(|| format!("failed to write {}", path.display()))
}

pub fn format_number(ticket: &Ticket, allocated_number: i64) -> String {
    let width = ticket.number_padding.max(0) as usize;
    let number = if width == 0 {
        allocated_number.to_string()
    } else {
        format!("{allocated_number:0width$}")
    };
    format!("{}{}{}", ticket.number_prefix, number, ticket.number_suffix)
}

fn group_thousands(value: i64) -> String {
    let digits = value.to_string();
    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (index, digit) in digits.bytes().enumerate() {
        if index > 0 && (digits.len() - index) % 3 == 0 {
            grouped.push(' ');
        }
        grouped.push(digit as char);
    }
    grouped
}

pub fn currency_exponent(currency: &str) -> u32 {
    Currency::from_code(currency)
    .and_then(|value| value.exponent())
    .unwrap_or(2)
    .into()
}

pub fn format_money(amount_minor: i64, currency: &str) -> String {
    let exponent = currency_exponent(currency);
    let divisor = 10i64.pow(exponent);
    let negative = amount_minor < 0;
    let absolute = amount_minor.abs();
    let major = group_thousands(absolute / divisor);
    let minor = absolute % divisor;
    let sign = if negative { "-" } else { "" };
    let amount = if minor == 0 {
        format!("{sign}{major}")
    } else {
        format!("{sign}{major},{minor:0width$}", width = exponent as usize)
    };

    match Currency::from_code(currency) {
        Some(value) => format!("{amount} {}", value.symbol()),
        None if currency.is_empty() => amount,
        None => format!("{amount} {currency}"),
    }
}

pub fn format_date(local_date: &str) -> String {
    let parts: Vec<&str> = local_date.split('-').collect();
    if parts.len() == 3 {
        format!("{}.{}.{}", parts[2], parts[1], parts[0])
    } else {
        local_date.to_owned()
    }
}
