use super::*;

const EDGE_INSET_MM: f64 = 2.0;
const MINOR_TICK_MM: f64 = 2.0;
const MEDIUM_TICK_MM: f64 = 4.0;
const MAJOR_TICK_MM: f64 = 6.0;
const HAIRLINE_MM: f64 = 0.2;

pub fn render_calibration_svg(
    page: PageSize,
    queue: &str,
    calibration: PrintCalibration,
) -> String {
    let width = page.width_mm.max(10.0);
    let height = page.height_mm.max(10.0);
    let calibration = calibration.normalized();
    let mut svg = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <svg xmlns="http://www.w3.org/2000/svg" width="{width}mm" height="{height}mm" viewBox="0 0 {width} {height}">
        <rect x="0" y="0" width="{width}" height="{height}" fill="white"/>
        <g id="print-calibration"{transform} fill="none" stroke="black">"#,
        transform = calibration_transform(calibration),
    );

    svg.push_str(&page_outlines(width, height));
    svg.push_str(&corner_marks(width, height));
    svg.push_str(&edge_rulers(width, height));
    svg.push_str(&centre_cross(width, height));
    svg.push_str(&measuring_bars(width, height));
    svg.push_str(&sheet_labels(width, height, queue, calibration));

    svg.push_str("\n</g>\n</svg>\n");
    svg
}

fn page_outlines(width: f64, height: f64) -> String {
    let inset = EDGE_INSET_MM;
    format!(
        r#"
        <rect x="0" y="0" width="{width}" height="{height}" stroke-width="{HAIRLINE_MM}"/>
        <rect x="{inset}" y="{inset}" width="{inner_width}" height="{inner_height}"
        stroke-width="{HAIRLINE_MM}" stroke-dasharray="1 1"/>
        <line x1="0" y1="0" x2="{width}" y2="{height}" stroke-width="{HAIRLINE_MM}" stroke-dasharray="2 2"/>
        <line x1="{width}" y1="0" x2="0" y2="{height}" stroke-width="{HAIRLINE_MM}" stroke-dasharray="2 2"/>"#,
        inner_width = (width - inset * 2.0).max(1.0),
        inner_height = (height - inset * 2.0).max(1.0),
    )
}

fn corner_marks(width: f64, height: f64) -> String {
    let arm = (width.min(height) * 0.12).clamp(3.0, 10.0);
    let mut marks = String::new();
    for (x, y, dx, dy) in [
        (0.0, 0.0, 1.0, 1.0),
        (width, 0.0, -1.0, 1.0),
        (0.0, height, 1.0, -1.0),
        (width, height, -1.0, -1.0),
    ] {
        marks.push_str(&format!(
                r#"
                <path d="M {x} {corner_y} L {x} {y} L {corner_x} {y}" stroke-width="0.4"/>"#,
                corner_x = x + arm * dx,
                corner_y = y + arm * dy,
        ));
    }
    marks
}

fn edge_rulers(width: f64, height: f64) -> String {
    let mut ruler = String::new();
    let mut position = 0.0;
    while position <= width + 0.001 {
        let length = tick_length(position);
        ruler.push_str(&format!(
                r#"
                <line x1="{position}" y1="0" x2="{position}" y2="{length}" stroke-width="{HAIRLINE_MM}"/>"#
        ));
        if is_major(position) && position > 0.001 && position < width - 0.001 {
            ruler.push_str(&ruler_label(position, MAJOR_TICK_MM + 3.0, position));
        }
        position += 1.0;
    }
    position = 0.0;
    while position <= height + 0.001 {
        let length = tick_length(position);
        ruler.push_str(&format!(
                r#"
                <line x1="0" y1="{position}" x2="{length}" y2="{position}" stroke-width="{HAIRLINE_MM}"/>"#
        ));
        if is_major(position) && position > 0.001 && position < height - 0.001 {
            ruler.push_str(&format!(
                    r#"
                    <text x="{x}" y="{y}" font-family="Noto Sans" font-size="2.4" fill="black"
                    stroke="none" text-anchor="start" dominant-baseline="middle">{label}</text>"#,
                    x = MAJOR_TICK_MM + 1.0,
                    y = position,
                    label = (position / 10.0) as i64,
            ));
        }
        position += 1.0;
    }
    ruler
}

fn ruler_label(x: f64, y: f64, value: f64) -> String {
    format!(
        r#"
        <text x="{x}" y="{y}" font-family="Noto Sans" font-size="2.4" fill="black"
        stroke="none" text-anchor="middle">{label}</text>"#,
        label = (value / 10.0) as i64,
    )
}

fn tick_length(position: f64) -> f64 {
    if is_major(position) {
        MAJOR_TICK_MM
    } else if (position / 5.0).fract().abs() < 0.001 {
        MEDIUM_TICK_MM
    } else {
        MINOR_TICK_MM
    }
}

fn is_major(position: f64) -> bool {
    (position / 10.0).fract().abs() < 0.001
}

fn centre_cross(width: f64, height: f64) -> String {
    let arm = (width.min(height) * 0.1).clamp(2.0, 8.0);
    let centre_x = width / 2.0;
    let centre_y = height / 2.0;
    format!(
        r#"
        <line x1="{left}" y1="{centre_y}" x2="{right}" y2="{centre_y}" stroke-width="0.3"/>
        <line x1="{centre_x}" y1="{top}" x2="{centre_x}" y2="{bottom}" stroke-width="0.3"/>
        <circle cx="{centre_x}" cy="{centre_y}" r="{arm}" stroke-width="0.3"/>"#,
        left = centre_x - arm * 1.6,
        right = centre_x + arm * 1.6,
        top = centre_y - arm * 1.6,
        bottom = centre_y + arm * 1.6,
    )
}

fn measuring_bars(width: f64, height: f64) -> String {
    let horizontal = measuring_span(width);
    let vertical = measuring_span(height);
    let start_x = (width - horizontal) / 2.0;
    let start_y = (height - vertical) / 2.0;
    let bar_y = height * 0.78;
    let bar_x = width * 0.78;
    format!(
        r#"
        <path d="M {start_x} {bar_y_top} L {start_x} {bar_y_bottom} M {start_x} {bar_y} L {end_x} {bar_y}
        M {end_x} {bar_y_top} L {end_x} {bar_y_bottom}" stroke-width="0.3"/>
        <text x="{label_x}" y="{label_y}" font-family="Noto Sans" font-size="2.6" fill="black"
        stroke="none" text-anchor="middle">{horizontal_label} mm</text>
        <path d="M {bar_x_left} {start_y} L {bar_x_right} {start_y} M {bar_x} {start_y} L {bar_x} {end_y}
        M {bar_x_left} {end_y} L {bar_x_right} {end_y}" stroke-width="0.3"/>
        <text x="{vertical_label_x}" y="{vertical_label_y}" font-family="Noto Sans" font-size="2.6"
        fill="black" stroke="none" text-anchor="middle"
        transform="rotate(-90 {vertical_label_x} {vertical_label_y})">{vertical_label} mm</text>"#,
        bar_y_top = bar_y - 1.5,
        bar_y_bottom = bar_y + 1.5,
        end_x = start_x + horizontal,
        label_x = width / 2.0,
        label_y = bar_y - 2.5,
        bar_x_left = bar_x - 1.5,
        bar_x_right = bar_x + 1.5,
        end_y = start_y + vertical,
        vertical_label_x = bar_x - 2.5,
        vertical_label_y = height / 2.0,
        horizontal_label = horizontal as i64,
        vertical_label = vertical as i64,
    )
}

fn measuring_span(available_mm: f64) -> f64 {
    let usable = (available_mm - EDGE_INSET_MM * 4.0).max(10.0);
    ((usable / 10.0).floor() * 10.0).max(10.0)
}

fn sheet_labels(width: f64, height: f64, queue: &str, calibration: PrintCalibration) -> String {
    let font_size = (width.min(height) * 0.045).clamp(2.0, 3.4);
    let lines = [
        format!("{} x {} mm", format_sheet_mm(width), format_sheet_mm(height)),
        if queue.trim().is_empty() {
            "no queue selected".to_owned()
        } else {
            queue.trim().to_owned()
        },
        format!(
            "offset {} / {} mm   scale {} / {} %",
            format_sheet_mm(calibration.offset_x_mm),
            format_sheet_mm(calibration.offset_y_mm),
            format_sheet_mm(calibration.scale_x_percent),
            format_sheet_mm(calibration.scale_y_percent),
        ),
    ];
    let mut labels = String::new();
    for (index, line) in lines.iter().enumerate() {
        labels.push_str(&format!(
                r#"
                <text x="{x}" y="{y}" font-family="Noto Sans" font-size="{font_size}" fill="black"
                stroke="none" text-anchor="middle">{line}</text>"#,
                x = width / 2.0,
                y = height * 0.2 + index as f64 * font_size * 1.4,
                line = escape_xml(line),
        ));
    }
    labels
}

fn format_sheet_mm(value: f64) -> String {
    let rounded = (value * 10.0).round() / 10.0;
    let text = format!("{rounded:.1}");
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}
