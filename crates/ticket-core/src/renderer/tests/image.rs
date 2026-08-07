use super::*;

#[test]
fn renders_image_field_inside_field_box() {
    let image_path =
    std::env::temp_dir().join(format!("ticket-renderer-image-{}.png", std::process::id()));
    std::fs::write(&image_path, b"fake png bytes").expect("image should be written");

    let ticket = sample_ticket("Image Ticket", 70.0, 50.0);
    let subticket = sample_subticket(&ticket);
    let fields = vec![image_field(
            &ticket,
            image_path.to_string_lossy().into_owned(),
    )];
    let context = sample_context(&subticket, 1);

    let svg =
    render_ticket_svg(&ticket, &fields, &subticket, &context).expect("ticket should render");

    let _ = std::fs::remove_file(image_path);
    assert!(svg.contains(r#"<clipPath id="field-clip-0">"#));
    assert!(svg.contains(r#"preserveAspectRatio="xMidYMid meet""#));
    assert!(svg.contains(r#"href="data:image/png;base64,"#));
    assert!(!svg.contains(r#"<text "#));
}

#[test]
fn renders_text_inside_field_box() {
    let ticket = sample_ticket("Small Ticket", 70.0, 50.0);
    let subticket = sample_subticket(&ticket);
    let fields = vec![overflowing_text_field(&ticket)];
    let context = sample_context(&subticket, 1);

    let svg =
    render_ticket_svg(&ticket, &fields, &subticket, &context).expect("ticket should render");

    assert!(svg.contains(r#"<clipPath id="field-clip-0">"#));
    assert!(svg.contains(r#"clip-path="url(#field-clip-0)""#));
    assert!(!svg.contains(r#"font-size="3.28mm""#));
    let font_size = rendered_font_size(&svg);
    assert!(font_size <= 3.28, "font size was {font_size}");
}

fn image_field(ticket: &Ticket, image_path: String) -> TicketField {
    let mut field = field_base(ticket, "image", FieldKind::Image);
    field.text = image_path;
    field.x_mm = 8.0;
    field.y_mm = 9.0;
    field.width_mm = 22.0;
    field.height_mm = 14.0;
    field.horizontal_alignment = "center".into();
    field.vertical_alignment = "center".into();
    field
}

fn overflowing_text_field(ticket: &Ticket) -> TicketField {
    let mut field = field_base(ticket, "long", FieldKind::Static);
    field.text = "Very long text that would otherwise overflow".into();
    field.font_size_pt = 36.0;
    field
}
