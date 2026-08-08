use super::*;
use crate::renderer::text::layout_text;

const LINE_ADVANCE_FACTOR: f64 = 1.18;
const ASCENT_FACTOR: f64 = 0.8;
const POINT_TO_MM: f64 = 25.4 / 72.0;

fn assert_close(actual: f64, expected: f64, what: &str) {
    assert!(
        (actual - expected).abs() < 1e-9,
        "{what}: expected {expected}, got {actual}"
    );
}

fn text_field(
    ticket: &Ticket,
    vertical_alignment: &str,
    horizontal_alignment: &str,
) -> TicketField {
    let mut field = field_base(ticket, "text", FieldKind::Static);
    field.x_mm = 5.0;
    field.y_mm = 10.0;
    field.width_mm = 40.0;
    field.height_mm = 30.0;
    field.font_size_pt = 12.0;
    field.vertical_alignment = vertical_alignment.into();
    field.horizontal_alignment = horizontal_alignment.into();
    field
}

#[test]
fn multi_line_layout_matches_the_editor_contract() {
    let ticket = sample_ticket("layout", 60.0, 80.0);
    let field = text_field(&ticket, "center", "center");
    let layout = layout_text(&field, "AB\nCD");

    let font_size = 12.0 * POINT_TO_MM;
    let block_height = font_size * (1.0 + LINE_ADVANCE_FACTOR);
    let block_top = 10.0 + (30.0 - block_height) / 2.0;

    assert_eq!(layout.lines, vec!["AB", "CD"]);
    assert_close(layout.font_size_mm, font_size, "font size");
    assert_close(
        layout.line_advance_mm,
        font_size * LINE_ADVANCE_FACTOR,
        "line advance",
    );
    assert_close(
        layout.first_baseline_mm,
        block_top + font_size * ASCENT_FACTOR,
        "first baseline",
    );
    assert_close(layout.anchor_x_mm, 5.0 + 40.0 / 2.0, "anchor x");
    assert_eq!(layout.anchor, "middle");
}

#[test]
fn every_vertical_alignment_uses_the_same_ascent() {
    let ticket = sample_ticket("layout", 60.0, 80.0);
    let font_size = 12.0 * POINT_TO_MM;
    let block_height = font_size * (1.0 + LINE_ADVANCE_FACTOR);
    let ascent = font_size * ASCENT_FACTOR;

    for (alignment, block_top) in [
        ("top", 10.0),
        ("center", 10.0 + (30.0 - block_height) / 2.0),
        ("bottom", 10.0 + 30.0 - block_height),
    ] {
        let field = text_field(&ticket, alignment, "left");
        let layout = layout_text(&field, "AB\nCD");
        assert_close(
            layout.first_baseline_mm,
            block_top + ascent,
            &format!("{alignment} baseline"),
        );
    }
}

#[test]
fn horizontal_alignment_picks_the_matching_svg_anchor() {
    let ticket = sample_ticket("layout", 60.0, 80.0);
    for (alignment, anchor, anchor_x) in [
        ("left", "start", 5.0),
        ("center", "middle", 25.0),
        ("right", "end", 45.0),
    ] {
        let field = text_field(&ticket, "top", alignment);
        let layout = layout_text(&field, "AB");
        assert_eq!(layout.anchor, anchor, "{alignment} anchor");
        assert_close(layout.anchor_x_mm, anchor_x, &format!("{alignment} x"));
    }
}

#[test]
fn italic_fields_render_an_italic_font_style() {
    let ticket = sample_ticket("layout", 60.0, 80.0);
    let subticket = sample_subticket(&ticket);
    let context = sample_context(&subticket, 1);
    let mut field = text_field(&ticket, "top", "left");
    field.text = "Sample".into();
    field.font_italic = true;

    let svg = render_ticket_svg(&ticket, &[field.clone()], &subticket, &context)
    .expect("italic field should render");
    assert!(svg.contains(r#"font-style="italic""#), "{svg}");

    field.font_italic = false;
    let svg = render_ticket_svg(&ticket, &[field], &subticket, &context)
    .expect("upright field should render");
    assert!(svg.contains(r#"font-style="normal""#), "{svg}");
}
